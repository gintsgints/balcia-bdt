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

So to convert specific BDT from oracle database to SQL scripts, one would use next command:

```shell script
balcia-bdt-v2.exe oracle AL01_PACKAGE_DEALS | balcia-bdt-v2.exe sql AL01_PACKAGE_DEALS
```

or to save specific BDT to CSV execute:

```shell script
balcia-bdt-v2.exe oracle AL01_PACKAGE_DEALS | ./target/debug/balcia-bdt-v2 csv write ./data/TEST/
```
