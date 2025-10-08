# Heltec WiFi LoRa 32 V3

## Board Info

- **Tier:** 3
- **Ariel OS Name:** `heltec-wifi-lora-32-v3`
- **Chip:** ESP32-S3
- **Chip Ariel OS Name:** `esp32s3`

### References

- [Manufacturer link](https://web.archive.org/web/20250807184214/https://heltec.org/project/wifi-lora-32-v3/)

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^usb-does-not-enumerate][^see-also-https-github-com-ariel-os-ariel-os-issues-903]|
|Wi-Fi|<span title="supported">✅</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-partitioning-support]|

<p>Legend:</p>

<dl>
  <div>
    <dt>✅</dt><dd>supported</dd>
  </div>
  <div>
    <dt>☑️</dt><dd>supported with some caveats</dd>
  </div>
  <div>
    <dt>🚦</dt><dd>needs testing</dd>
  </div>
  <div>
    <dt>❌</dt><dd>available in hardware, but not currently supported by Ariel OS</dd>
  </div>
  <div>
    <dt>–</dt><dd>not available on this piece of hardware</dd>
  </div>
</dl>
<style>
dt, dd {
  display: inline;
}
</style>

[^usb-does-not-enumerate]: USB does not enumerate.
[^see-also-https-github-com-ariel-os-ariel-os-issues-903]: See also: https://github.com/ariel-os/ariel-os/issues/903.
[^requires-partitioning-support]: Requires partitioning support.