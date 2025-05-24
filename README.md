# csvgen - simple CSV file generator

## Usage
```
csvgen --columns <columns> --out <out> --rows <rows>

Options:
  -c, --columns <columns>  Column names (delimiter = ',')
  -o, --out <out>          Output filepath
  -r, --rows <rows>        Rows count
  -h, --help               Print help
  -V, --version            Print version
```

## Example
### Command
```
./csvgen -o ./test.csv -r 10 -c "a1,a2,a3" 
```
### *test.csv* content
```csv
a1;a2;a3
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
Y9DOafw4KIj1eMwT;a9zaCXPz4L0RzoZM;1SfeFJSOd5rLqrot
```