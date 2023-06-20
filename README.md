# Balcia data table management application

## Usage

Application uses stdin & stdout to input output data. Simplest call
prints provided CSV data withing path to stdout:

```shell script
balcia-bdt-v2.exe csv read ./data/TT/
```

You can check possible options by issuing:

```shell script
balcia-bdt-v2.exe --help
```

or for specific command:

```shell script
balcia-bdt-v2.exe sql --help
```

So to convert specific BDT from oracle database to SQL scripts, one would use next command
(make sure you set oracle connection environment variables before)
:

```shell script
balcia-bdt-v2.exe oracle --help
balcia-bdt-v2.exe oracle AL01_PACKAGE_DEALS | balcia-bdt-v2.exe sql AL01_PACKAGE_DEALS
```

or to save specific BDT to CSV execute:

```shell script
balcia-bdt-v2.exe oracle AL01_PACKAGE_DEALS | ./target/debug/balcia-bdt-v2 csv write ./data/TEST/
```

You can extract multiple BDTs using mask:

```shell script
balcia-bdt-v2.exe oracle PRODUCT_%% >./data/PRODUCT.json
```

## Getting BDT from oracle DB and generating oracle load scripts from it

First load BDT from oracle to json

```shell script
balcia-bdt-v2.exe oracle LT_TRAVEL_INDIVIDUAL_RISK_SUM_INSURED_FACTOR >./data/LT_TRAVEL_INDIVIDUAL_RISK_SUM_INSURED_FACTOR.json
```

Then Generate load SQL scripts

```shell script
type ./data/LT_TRAVEL_INDIVIDUAL_RISK_SUM_INSURED_FACTOR.json | balcia-bdt-v2 sql LT_TRAVEL_INDIVIDUAL_RISK_SUM_INSURED_FACTOR
```


## Load CSV data to SQLite DB

```shell script
balcia-bdt-v2.exe oracle TT_EMPREKIS_DATA >./data/TT_EMPREKIS_DATA.json
type .\data\TT_EMPREKIS_DATA.json | balcia-bdt-v2 sqlite >.\data\TT_EMPREKIS_DATA\create_emprekis.sql
type .\data\TT_EMPREKIS_DATA.json | balcia-bdt-v2 csv data .\data\TT_EMPREKIS_DATA\data.csv TT_EMPREKIS_DATA
sqlite3 ./data/bdt.db
.read create_emprekis.sql
.separator ,
.import --skip 1 ./data/TT_EMPREKIS_DATA/data.csv TT_EMPREKIS_DATA
```

