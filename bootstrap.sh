#!/bin/sh
bootimage --target x86_64-blog_os && qemu-system-x86_64 -drive format=raw,file=bootimage.bin