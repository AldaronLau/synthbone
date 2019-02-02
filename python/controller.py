# Python file for detecting Joystick and Button Input.
import time

# Import SPI library (for hardware SPI) and MCP3008 library.
import Adafruit_GPIO.SPI as SPI
import Adafruit_MCP3008

# Software SPI configuration:
CLK  = 11
MISO = 9 # DOUT
MOSI = 10 # DIN
CS   = 8
mcp = Adafruit_MCP3008.MCP3008(clk=CLK, cs=CS, miso=MISO, mosi=MOSI)

# Hardware SPI configuration:
# SPI_PORT   = 0
# SPI_DEVICE = 0
# mcp = Adafruit_MCP3008.MCP3008(spi=SPI.SpiDev(SPI_PORT, SPI_DEVICE))

# Main program loop.
while True:
    # Detect X Axis
    jsx = mcp.read_adc(0)

    # Detect Y Axis
    jsy = mcp.read_adc(1)

    print('X: ' + str(jsx) + ', Y: ' + str(jsy))
