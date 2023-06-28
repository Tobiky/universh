
```none
// single line comment
/* multi line comment */ 

// semicolons (to be) optional everywhere
// forces expressions into statements

// function definition with optional types
// TODO: static or dymaic typing?
// return on ending expression or return statement
fnc add(p1: int, p2: int): int {
    p1 + p2
    // or: return p1 + p2;
}

// no paremeters short definition
fnc empty: int {
    2 * 3
}

// command options with flags, (positional) arguments, and an optional rest variable
// allows specification on flags to handle schemas/verification among other things
// all values not supplied not handled by option specification are put into the rest variable
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
sql = @import("sql.ush");

// run the file as inline, later "run test"
@run(test);

// define and assign variables
k = 3;

// sql export is a function to create a connection
connection = sql(asdfadsfasdfadsf);

// connection object defines a meta call to a DSL plugin with passable options
// $k means to allow/pass access to variable/value of k
x = @connection[compression=method]{
    SELECT x FROM l where x.y_point = $k
};

// inherently starts as a background task (async child process)
screens = cmd xrandr --quiet;

// process is an object, behaves similarly to Futures
second = screens.stdout().split().last().parse_int();

// inherent string interpolation and prioritisation over other expressions
value = "${second}";
p1 = cmd SCREEN="${second}" polybar barX;
if p1.code == 0 {
    cmd echo "launched successfully";
};

// if's are also expressions
// only returns the last expression in the body if there is one
// otherwise forms a standard if-statement
echo_process = if p1.code != 1 {
    cmd echo "polybar did not start"
}

// two processes with piping inbetween
piped_result = cmd xrandr --quiet | string split 0 24;

// creating a range (iterator)
range = 1..2;

// for loops can also be used for list comprehension
// aka as an expression, just like if's
my_list = for x in [1, 2, 3] {
    x * 2
};

// creating a slice with index and range syntax
slice = my_list[0..1];

// use a defined intrinsic on a type, a filter for lists
filtered = slice(|x| even(x));

// defining a type
// or type list
type List {
    list = []

    // defininig an intrinsic, similar to: dunderscore function in python, prototype funtion in javascript
    @intrinsic(call)
    // can have any name
    fnc call_intrinsic(
        self: List /* or this, or object, etc */,
        func: fnc (any): bool /* or args, or argv, etc */)
        : List
    {
        return self.filter(func);
    }

    @intrinsic(index)
    fnc index_intrinsic(self: List, index: any): any {
        return self.list[index];
    }

    // TODO: interops, meta, and plugin syntax
}
```
