
```none
// single line comment
/* multi line comment */ 

// semicolons optional

fnc add(p1: int, p2: int): int {
    return p1 + p2
}

fnc empty: int {
    return 2 * 3
}

cmd add with 
    flag p1 = {
        long = 'jobs',
        short = 'j',
        length = 4,
        seperator = [',', ' '],
        types = [int, str, char, int],
        optional = true
    },
    arg a1 = ...,
    flag p2 = {...},
    arg a2 = ...,
    rest r
{
    // ...
}

// import definitions, later "import sql"
@import(sql);

// run the file as inline, later "run test"
@run(test);

// define and assign variables
k = 3;

connection = sql(asdfadsfasdfadsf)

x = @connection[compression=method]{
    SELECT x FROM l where x.y_point = $k
}


screens = cmd xrandr --quiet;
second = screens.stdout().split().last().parse_int()
value = "${second}"
p1 = cmd SCREEN="${second}" polybar barX
if p1.code == 0 {
    cmd echo "launched successfully"
}

piped_result = cmd xrandr --quiet | string split 0 24
```
