# Language Syntax of Rain Language

This is file for showcasing the syntax of my programming language aka. Rain. Syntax is inspired from many languages, heavily Jai, Javascript, C++, Rust, V and Wren.

Enjoy!

## Main function, everything starts from here:

```c++
main :: () {
    println("Hello world!");
}
```

## Comments:

In rain we accept only line comments: `// comment`

## Variables:

### Mutable variable

```c++
x := 5;
```

### Constant

```c++
y :: 6;
```

Specify variable like this:

```c++
foo: int = 65;
bar := float(3); // will be turned into 3.0
```

You can initialize multiple variables in the same line:

```c++
x := 5, y := 5;
```

## Data Types:

In Rain we have most of the known data types, known in diffrent languages.

Integers:
| Signed | Unsigned |
| -      |    -     |
| `i8`   |   `u8`   |
| `i16`  |  `u16`   |
| `int`  |  `u32`   |
| `i64`  |  `u64`   |

Float:
- `f32`
- `f64`

Text:
- `char`
- `string`

Others:
- `struct`
- `array`
- `fiber`
- `enum`


## Functions:

```c++
hello :: (name) {
    println("Hello {name}!");
}

hello("Sky"); // > "Hello Sky!"

squared :: (x) {
    x ** 2
}

println("The squared of 5 is: { squared(5) }") // > "The squared of 5 is: 25"

squared_by :: (x, y = 2) int {
    return x ** y;
}
```

## Arrays:

Array can be defined using `[]` and typing the values:

```c++
numbers := [0, 1, 2, 3, 4];
```
Arrays can only have the same type of value, this means that if you are using integers in the array, then you can only use integers in that array, basically you can't specify values that don't match the array type.

You can also create an empty array with a specfied type:
```
names : string[5] = [];

names[0] = "Evry";
names[1] = "Daniel";
names[2] = "Chandler";
```

## Objects:

In Rain, we have objects to create variables that can hold diffrent types of values.

```c++
z :: {
    foo: 1,
    bar: x,
    baz: 2
}
```

## Structs:

```c++
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

```c++
foo :: fiber {
    println("Fiber called");
}

foo.call(); // > "Fiber called!"
```

Yielding:

```c++
bar :: fiber {
    println("passed to yield: {foo.call()}");
}

foo.yield("hello, world!"); // > "passed to yield: hello, world!"
```

## Operators:

### `::`

Initializator or `::` can be used to create a function, a struct or a constant that, like a struct and function, can't be modified after creation.

```c++
x: int : 5

StructName : FuncName : () {
    // do something...
}
```

### `:=`

The second initializator (`:=`), also known as the walrus operator, creates a modifiable variable:

```c++
x := 5;

x: int = 5;

x: int[] = [];
```

As you can see you can also provide the variable type after the colon.

## Import and Modules

Modules in Rain work really simple.

In Rain, modules are separeted into 3 types:
1. Global/standard library modules
2. Local modules (in around same location as your project)
3. Source files (with `.rain` file extension)

<br>

### Global/standard library modules

To use global modules, like the standard library, you need to provide only the name of the module. Here is an example:
```c++
#import math

main :: () {
    println("The numbers of π are {math.pi}");
}

// Outputs:
//   > The numbers of π are 3.1416...
```

### Local modules
To create local modules, first create a folder which the module's source files are going to be inside. Import it using `#import` preprocessor followed by the name of the folder.

Remember that functions and variables must use `pub` keyword to be accessible to the source file that later will import that module.

in foo/file.rain:

```c++
#import semver
version := semver.Version(0, 1, 0, semver.ALPHA);

pub hello :: () {
    println("foo");
    println("module, {version.str()}");
}
```
In main.rain file:

```c++
#import foo

main :: () {
    foo.hello();
	// > "foo"
    // > "module, v0.1.0-alpha"
}
```

### Source files

The documentation won't provide examples of using source files, but it quite says itself. You just create a source file to be imported and import it by the path to the file: `#import "file.rain"`.

---
### Import keywords:

### Access specifier:

### `pub`

By default if you import a local file, and try to get a variable or a function from that file, you will get a message that _**the variable or the function does not exist**_. The thing is, is that all imported modules/files are by default private, which means that they are not in the global, public scope. To use them, you have to use the `pub` keyword.

`pub` tells that the variable, function, struct, or etc. can be accessed from that file using `#import`.

Public functions or are especially useful when you want to e.g. print a private value which you don't want the user to access by itself. To use it, write `pub` before the variable, struct, or function you are creating in a module.


### `as` keyword

Use `as` keyword when you are importing a module, but want the variables and functions be in an namespace. Example:

```c++
#import "math" as Math;

main :: () {
    println("The numbers of π are { Math.pi }");
}

// Outputs:
//   > The numbers of π are 3.1416[...]

```

If you provide wildcard (`*`) instead of a name, it will try to make all the variables and functions available as if they were a part of the source file they get imported to e.g.:
```c++
#import "math" as *;

main :: () {
    println("The numbers of π are { pi }");
}

// Outputs:
//   > The numbers of π are 3.1416...

```

This can be used if you know that the variables and functions of the source file will not collide with the properties and functions from the module, otherwise Rain will give an error you about that:
```c++
#import "math" as *;

pi := 3.14

main :: () {
    println("The numbers of π are { pi }");
}

// Compiler output:
// > error main.rain:3:0: Variable `pi` cannot be redefined as it is already defined (possibly from module "math").
// > pi := 3.14
// > ^^^^^^^^^^
```

## Conditional statements

### If and Else statement:

```c++
x := 3;
y := 5;

if y == x + 2 {
    println("This is true!");
} else {
    println("This is false");
}
```

`else if` also does exist, but we won't include an example for that, as its function works in same principle as in the upper example

### For statement:

For can be used to loop through arrays:

```c++
number := [0, 1, 2, 3, 4, 5];

for num in numbers {
    println(num);
}

names := ["Sam", "Peter"]

// for element, i (iter num (optional)) in array_element
for name, i in names {
    println("{i}) {name}");
    // > 0) Sam
    // > 1) Peter
}

// range numbers
for i in 0 .. 5 {
    print(i);
}
// > 01234
```

### Conditional loop

Rain uses the power of `for` keyword also to perform conditional loops. The feature is commonly used with the keyword `while` loop in other programming languages:
```c++
sum := 0
i := 0

for i <= 100 {
    sum += i;
    i++;
}
println(sum); // > 5050
```
<!-- TODO -->
## References and heap

Here is an example of how to get change variables value by getting its' reference:
```c++
modify :: (&x) {
	x = 10;
}

x := 6;

println(x);
// > 6

modify(*x); // variable's address is used to find the location of the value which later gets modified

println(x);
// > 10


```

## `match`

Some languages call it `switch`.

Here is an example:
```c++
#import io

print("Please write a fruit: ");
input := io.input();

match input {
    "apple" => {
        println("🍎!");
    },
    "orange" || "clementine" => {
        println("🍊!");
    }
    "pear" => {
        println("🍐!");
    }
    _ => {
        println("Hmmm, fruit \"{input}\" does not have an emoji...");
    }
}
```

## `once`
`once` keyword is a feature which will set the lifetime of a variable or a function to be dead/killed once the variable is read or when a function is runned.

```c++
once foo :: () {
	println("Outputing last time!");
}

foo();
// > "Outputing last time!"

foo();
// > error: function `foo` is no longer as it is using `once`.

```

Same example applies to once variables when they are read, this means that you can still modify them multiple times before it dies.

<!-- TODO -->
## Enums (`enum`)

```c++
State :: enum {
    READY, // 0
    STOPPED, // 1
    ERROR // 2
}

```
