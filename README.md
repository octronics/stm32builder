stm32builder - A tool to build stm32 HAL
========================================

This project provides a rust tool to help writing HAL (hardware abstraction
layer) for all STM32 microcontrollers.

## What is it providing?

 A library to decode the stm32 identification number, and to generate a device
 object containing relevant device informations.

 A command line tool (mostly useful to development) that allow to:
  - `decode` a device identification number,
  - `show` the device informations found on a device file that match a device
    identification number,
  - `print` like `show` but output yaml serialized rust object.
