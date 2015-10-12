# Switcher #

This is a very minimal framework for a first-stage bootloader for low-power ARM
processors. This is designed to help a system choose between multiple available
bootable images and then boot it.

## Operation ##

The general assumption is that the system will have multiple bootable images
(typically two) and the newer, working image of the two will be booted. In
order to be considered for boot, the image must have a valid checksum, it must
not be marked as having failed to boot, and it must have at least one available
boot attempt (images can be given, at most, four attempts).  When an image is
booted, the number of available attempts is decremented if it hasn't been
marked as having previously successfully booted. Once booted, images must mark
themselves as having successfully booted. Otherwise, the next time through the
boot process, the number of available attempts will be decremented, eventually
exhausting the available attempts and preventing that image from being used.

Examples of this process can be found in [examples][examples].

[examples]: examples

## Flash Layout ##

The layout of the flash must be compatible with the internal structures in
order to use this framework. The image header must be placed immediately after
the image it defines. This is to ease the process of checking an image's
checksum. The first element in the struct is the image's checksum. Since this
butts up against the image, the checksum can be validated by simply running the
image plus its checksum as one block of data through the [CRC][crc]. If the
image is valid, the result of the CRC will be 0.

[crc]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
