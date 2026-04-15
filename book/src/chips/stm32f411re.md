# STM32F411RE

## Chip Info

- **Ariel OS Name:** `stm32f411re`

## Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="needs testing">🚦</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^a-more-complete-clock-configuration-needs-to-be-provided]|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="not available on this piece of hardware">–</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^unsupported-heterogeneous-flash-organization]|

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
<div class="support-matrix-container">
<table class="support-matrix">
  <thead>
    <tr>
      <th colspan="3">Board</th>
      <th colspan="13">Functionality</th>
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
      <th>Ethernet</th>
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
	    <td rowspan="2"><a href="../boards/st-nucleo-f411re.html">ST NUCLEO-F411RE</a></td>
	  </tr>
	  <tr>
	    <td><code>st-nucleo-f411re</code></td>
		<td style="text-align: center;">3</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="supported">✅</td>
		  <td class="support-cell" title="needs testing">🚦</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="not available on this piece of hardware">–</td>
		  <td class="support-cell" title="available in hardware, but not currently supported by Ariel OS">❌</td>
      </tr>
	  </tbody>
  </tbody>
</table>
</div>
<style>
.support-matrix-container {
  overflow: auto;
  max-height: 60vh;
}
.support-matrix thead {
  z-index: 3;
  position: relative;
}
/* Makes the row with column names sticky */
.support-matrix thead tr:last-child {
  position: sticky;
  top: 0;
  background-color: var(--table-header-bg);
}
.support-matrix th:first-child {
  background-color: inherit;
}
/* Makes the first column sticky */
.support-matrix thead tr:last-child th:first-child,
.support-matrix tbody tr:first-child td:first-child {
  position: sticky;
  left: 0;
  z-index: 1;
}
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



[^a-more-complete-clock-configuration-needs-to-be-provided]: A more complete clock configuration needs to be provided.
[^unsupported-heterogeneous-flash-organization]: Unsupported heterogeneous flash organization.