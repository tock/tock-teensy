#include <tock.h>

int serial_read_once(char* buf, size_t len);
int serial_read(char* buf, size_t len);
int serial_write(char* buf, size_t len);

// Set the buffer that xmodem should fill with a transfer.
void xmodem_set_buffer(char* buf, size_t len);

// The callback that indicates an xmodem transfer completed.
// After this callback is issued, the xmodem library will wait
// until a new buffer is set with xmodem_set_buffer  before accepting
// a new transfer.
typedef void xmodem_callback(char* buf, int len, int error);
void xmodem_set_callback(subscribe_cb buffer_filled);

static char* xmodem_buf = NULL;
static size_t xmodem_buf_len = 0;
static uint8_t xmodem_blockno = 0;

void xmodem_set_buffer(char* buf, size_t len) {
  xmodem_buf = buf;
  xmodem_buf_len = len;
  xmodem_blockno = 1;
}


enum {
        SOH = 0x01,   // Start Of Header
        ACK = 0x06,   // Acknowledge (positive)
        NAK = 0x15,   // Acknowledge (negative)
        EOT = 0x04,   // End of transmission
        PAYLOAD_SIZE = 128,

        ARMBASE = 0x8000
};

// Performs a single UART read into buf. This read may
// read fewer than len bytes.
int serial_read_once(char* buf, size_t len) {
  int read_len = 0;
  void read_callback(int rlen,
                     __attribute__ ((unused)) int unused1,
                     __attribute__ ((unused)) int unused2,
                     void* ud) {
    *((bool*)ud) = true;
    read_len = rlen;
  }

  int ret = allow(0, 0, buf, len);
  bool done = false;
  if (ret < 0)  return ret;
  ret = subscribe(0, 0, read_callback, &done);
  if (ret < 0)  return ret;
  ret = command(0, 2, len);
  if (ret < 0)  return ret;
  yield_for(&done);
  return read_len;
}

// Reads until buf is filled with len bytes.
int serial_read(char* buf, size_t len) {
  size_t index;
  for (index = 0; index < len;) {
    size_t left = len - index;
    size_t count = serial_read_once(buf + index, left);
    index += count;
  }
  return (int)index;
}

int serial_write(char* buf, size_t len) {
  int write_len = 0;
  void write_callback(int wlen,
                     __attribute__ ((unused)) int unused1,
                     __attribute__ ((unused)) int unused2,
                     void* ud) {
    *((bool*)ud) = true;
    write_len = wlen;
  }
  bool done = false;

  int ret = allow(0, 1, buf, len);
  if (ret < 0) return ret;

  ret = subscribe(0, 1, write_callback, &done);
  if (ret < 0) return ret;

  ret = command(0, 1, len);
  yield_for(&done);
  return write_len;
}


static unsigned char serial_read_byte_timeout(uint32_t timeout) {
  bool byte_read = false;
  bool timeout = false;
  bool done = false;
  uint8_t byte = 0;
  tock_timer_t timer;
  void read_callback(int rlen,
                      __attribute__ ((unused)) int unused1,
                      __attribute__ ((unused)) int unused2,
                     void* ud) {
    done = true;
    if (rlen == 1) {
      byte_read = true;
    }
  }
  void timer_callback(int wlen,
                      __attribute__ ((unused)) int unused1,
                      __attribute__ ((unused)) int unused2,
                     void* ud) {
    done = true;
    timeout = true;
  }

  timer_in(timeout, timer_callback, NULL, &timer);

  yield_for(&done);

  unsigned t0 = timer_tick();
        // while uart not ready, just keep going; nak every 4 sec
        while(((uart_lcr() & 0x01) == 0)) {
                unsigned t = timer_tick();
                if ((t - t0) >= 4000000) {
                        uart_send(NAK);
                        t0 = t;
                }
        }
        return uart_recv();
}

void notmain ( void ) {

        /*
         * 132 byte packet.  All fields are 1 byte except for the 128 byte data
         * payload.
         *              +-----+------+----------+--....----+-----+
         *              | SOH | blk# | 255-blk# | ..data.. | cksum |
         *              +-----+------+----------+--....----+-----+
         * Protocol:
         *      - first block# = 1.
         *  - CRC is over the whole packet
         *  - after all packets sent, sender transmits a single EOT (must ACK).
         */
        unsigned char block = 1;
        unsigned addr = ARMBASE;
        while (1) {
                unsigned char b;

                // We received an EOT, send an ACK, jump to beginning of code
                if((b = getbyte()) == EOT) {
                        uart_send(ACK);
                        BRANCHTO(ARMBASE);
                        return; // NOTREACHED
                }

                /*
                 * if first byte is not SOH, or second byte is not the
                 * expected block number or the third byte is not its
                 * negation, send a nak for a resend of this block.
                 */
                if(b != SOH
                || getbyte() != block
                || getbyte() != (0xFF - block)) {
                        uart_send(NAK);
                        continue;
                }

                // get the data bytes
                int i;
                unsigned char cksum;
                for(cksum = i = 0; i < PAYLOAD_SIZE; i++) {
                        cksum += (b = getbyte());
                        PUT8(addr+i, b);
                }

                // Checksum failed: NAK the block
                if(getbyte() != cksum)
                        uart_send(NAK);
                // Commit our addr pointer and go to next block.
                else {
                        uart_send(ACK);
                        addr += PAYLOAD_SIZE;
                        block++;
                }
        }
}
