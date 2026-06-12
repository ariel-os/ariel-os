/* PSoC variant of linkme.x.
 *
 * psoc-rs's `psoc.x` hardcodes the load address of `.data` to immediately
 * follow `.rodata` (`AT(__FLASH_LMA + __erodata - ORIGIN(FLASH))`), rather
 * than letting the linker flow the LMA via `AT>FLASH` like cortex-m-rt's
 * upstream `link.x`. As a consequence, any extra `> FLASH` sections injected
 * between `.rodata` and `.data` (as the default `linkme.x` does with
 * `INSERT AFTER .rodata`) get a VMA that collides with `.data`'s fixed LMA.
 *
 * To avoid the overlap we place the linkme sections at an explicit address
 * right after `.data`'s flash image (`__sidata = LOADADDR(.data)`, plus the
 * size of `.data`). They hold read-only data that is consumed in place from
 * flash, so VMA == LMA is fine. Keeping the per-symbol output sections lets
 * the linker synthesise the `__start_*`/`__stop_*` encapsulation symbols.
 */
SECTIONS {
  linkme_INIT_FUNCS (__sidata + SIZEOF(.data)) : { KEEP(*(linkme_INIT_FUNCS)) } > FLASH
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

INSERT AFTER .data

/* without this, the linker makes `__sdata` point to `0x800xxxx` but `__edata` point
   to `0x1000xxx`, making the startup code .data initialization fail
*/
__sdata = ADDR(.data);
__edata = __sdata + SIZEOF(.data);
