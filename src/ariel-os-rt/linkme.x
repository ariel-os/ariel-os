SECTIONS {
  linkme_INIT_FUNCS : { KEEP(*(linkme_INIT_FUNCS)) } > FLASH
  linkm2_INIT_FUNCS : { KEEP(*(linkm2_INIT_FUNCS)) } > FLASH
  linkme_EMBASSY_TASKS : { KEEP(*(linkme_EMBASSY_TASKS)) } > FLASH
  linkm2_EMBASSY_TASKS : { KEEP(*(linkm2_EMBASSY_TASKS)) } > FLASH
  linkme_USB_BUILDER_HOOKS : { KEEP(*(linkme_USB_BUILDER_HOOKS)) } > FLASH
  linkm2_USB_BUILDER_HOOKS : { KEEP(*(linkm2_USB_BUILDER_HOOKS)) } > FLASH
  linkme_SENSOR_REFS : { KEEP(*(linkme_SENSOR_REFS)) } > FLASH
  linkm2_SENSOR_REFS : { KEEP(*(linkm2_SENSOR_REFS)) } > FLASH
  linkme_THREAD_FNS : { KEEP(*(linkme_THREAD_FNS)) } > FLASH
  linkm2_THREAD_FNS : { KEEP(*(linkm2_THREAD_FNS)) } > FLASH
}

INSERT AFTER .rodata
