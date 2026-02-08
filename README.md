# Helper tool to download and import tax rate data for Switzerland

The Swiss Federal Tax Administration provides a web interface to retrieve
information about tax rates, multipliers and deductions, available at
[swisstaxcalculator.estv.admin.ch](https://swisstaxcalculator.estv.admin.ch/#/taxdata/tax-rates).
It's a comprehensive data set at all administrative levels (federal, cantonal
and communal), for tax years ranging from 2010 to 2025 (at the time of writing).
Unfortunately, exporting data from this interface isn't very convenient, as it
only allows exporting one subset at a time into non-portable `.xlsx` format,
even though it internally sends JSON data to the browser.

This helper tool fixes that: it provides a convenient way to download all the
data in JSON format, together with a schema. I'm not including the data itself
as the default
[terms and conditions](https://www.admin.ch/gov/en/start/terms-and-conditions.html#1938362905)
unfortunately require prior written consent of the copyright holder (the Swiss
federal authorities), rather than offering them in a customary open data
license. I also haven't found this data set on the
[Swiss open data portal](https://opendata.swiss/en) - if you're reading this and
working at a Swiss administration please consider adding it there or clarifying
the license.
