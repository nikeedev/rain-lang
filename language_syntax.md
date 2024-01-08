# Language Syntax of Rain Language

This is file for showcasing the syntax of my programming language aka. Rain. Syntax is inspired from many languages, heavily Jai, Javascript, C++, and Wren.

Enjoy!

## Main function, everything starts from here:

```rain
main :: () {
    println("Hello world!");
}
```

## Comments:

In rain we accept only line comments: `// comment`

## Variables:

### Mutable variable

```rain
x := 5;
```

### Constant

```rain
y :: 6;
```

Specify variable like this:

```rain
foo: int = 65;
bar := float(3); // will be turned into 3.0
```

You can initialize multiple variables in the same line:

```rain
x := 5, y := 5;
```

## Data Types:

In Rain we have most of the known data types, known in diffrent languages.

Integers:
| Signed | Unsigned |
| - | - |
| `i8` | `u8` |
| `i16` | `u16` |
| `int` | `u32` |
| `i64` | `u64` |

Float:
- `f32`
- `f64`

Text:
- `char`
- `string`

## Functions:

```rain
hello :: (name) {
    println("Hello ${name}!");
}

hello("Sky"); // > "Hello Sky!"

squared :: (x) {
    x ** 2
}

println("The squared of 5 is: ${squared(5)}") // > "The squared of 5 is: 25"

squared_by :: (x, y = 2) int {
    return x ** y;
}
```

## Arrays:

```rain
numbers := [0, 1, 2, 3, 4];

names : string[5] = [];

names[0] = "Evry";
names[1] = "Daniel";
names[2] = "Chandler";
```

## Objects:

```rain
z :: object {
    "foo": 1,
    "bar": x,
    "baz": 2
}
```

## Structs:

```rain
Vector2 :: struct {
    x: int,
    y: int,
}

a := Vector2 { 6, 5 };

println(a.x); // > 6

println(a); // > Vector2 { x: 6, y: 5 }

Vector2 : magnitude : () {
    return int(sqrt(this.x * this.x + this.y * this.y));
}

println(a.magnitude()); // > 7.8
```

## Fibers:

Create a fiber like this:

```rain
foo :: fiber {
    println("Fiber called");
}

foo.call(); // > "Fiber called!"
```

Yielding:

```rain
bar :: fiber {
    println("passed to yield: {foo.call()}");
}

foo.yield("hello, world!"); // > "passed to yield: hello, world!"
```

## Operators:

### `::`

Initializator or `::` can be used to create a function, a struct or a constant that, like a struct and function, can't be modified after creation.

```rain
x: int : 5

StructName : FuncName : () {
    // do something...
}
```

### `:=`

The second initializator (`:=`), also known as the walrus operator, creates a modifiable variable:

```rain
x := 5;

x: int = 5;

x: int[] = [];
```

## Import and Modules

in file.rain:

```rain
pub foo :: () {
    println("foo");
}
```

in main.rain file:

```rain
#import "file.rain"

main :: () {
    foo();
}
```

## Conditional statements

If statement:

```rain
x := 5;
y := 3;

if (x == y + 2) {
    println("This is true!");
} else {
    println("This is false");
}
```

For statement:

For can be used to loop through arrays:

```rain
number := [0, 1, 2, 3, 4, 5];

for num in numbers {
    println(num);
}

names := ["Sam", "Peter"]

for i, name in names {
    println("{i}) {name}");
    // > 0) Sam
    //   1) Peter
}

// range numbers
for i in 0 .. 5 {
    print(i);
}
// > 01234

// or conditional looping, also known as `while` loop in other languages
sum := 0
i := 0

for i <= 100 {
    sum += i;
    i++;
}
println(sum); // > 5050
```

## (TODO) References and heap

```rain
x := 6;
```
