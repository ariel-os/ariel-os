# ESP32-C3

## Chip Info

- **Ariel OS Name:** `esp32c3`

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>[^no-generic-usb-peripheral]|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="supported">✅</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
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



## Boards

Boards using this chip.

<!-- This table is auto-generated. Do not edit manually. -->
<table class="support-matrix">
  <thead>
    <tr>
      <th colspan="3">Board</th>
      <th colspan="12">Functionality</th>
    </tr>
    <tr>
      <th>Manufacturer Name</th>
      <th><a href="../build-system.html#laze-builders">laze builders</a></th>
      <th>Tier</th>
      <th>Debug Output</th>
      <th>Logging</th>
      <th>GPIO</th>
      <th>I2C Controller Mode</th>
      <th>SPI Main Mode</th>
      <th>UART</th>
      <th>User USB</th>
      <th>Ethernet over USB</th>
      <th>Wi-Fi</th>
      <th>Bluetooth Low Energy</th>
      <th>Hardware Random Number Generator</th>
      <th>Persistent Storage</th>
    </tr>
  </thead>
  <tbody>
	<tbody class="odd">
      <tr>
	    <td rowspan="2"><a href="../boards/espressif-esp32-c3-lcdkit.html">Espressif ESP32-C3-LCDkit</a></td>
	  </tr>
	  <tr>
	    <td><code>espressif-esp32-c3-lcdkit</code></td>
		<td style="text-align: center;">1</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="available in hardware, but not currently supported by Ariel OS">❌</td>
      </tr>
	  </tbody>
	<tbody class="even">
      <tr>
	    <td rowspan="2"><a href="../boards/espressif-esp32-c3-devkit-rust-1.html">Espressif ESP32-C3-DevKit-RUST-1</a></td>
	  </tr>
	  <tr>
	    <td><code>espressif-esp32-c3-devkit-rust-1</code></td>
		<td style="text-align: center;">3</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="available in hardware, but not currently supported by Ariel OS">❌</td>
      </tr>
	  </tbody>
  </tbody>
</table>
<style>
@media (min-width: 1920px) {
  .support-matrix {
    position: relative;
    left: 50%;
    transform: translate(-50%, 0);
  }
}
.support-cell {
  text-align: center;
}
tbody.even td { background-color: var(--bg); }
tbody.odd td { background-color: var(--table-alternate-bg); }
</style>

<p>Key:</p>

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



[^no-generic-usb-peripheral]: No generic USB peripheral.
[^requires-partitioning-support]: Requires partitioning support.