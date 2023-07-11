fn main() {
    characteristics_of_object_oriented_languages();
    using_trait_objects_that_allow_for_values_of_different_types();
}

fn characteristics_of_object_oriented_languages() {
    //Rust has encapsulation. It allows certain elements of `objects` to be hidden from the end user.
    // This is nothing new, it is just showing the similarities between the concepts know to be a
    // part of OOP.

    //Polymorphism is actually not specifically about inheritance. It is a more general concept that
    // means code that can work with data of multiple types. Rust does not explicitly have
    // inheritance. Instead it uses the generics system to abstract over the different possible
    // types, this is apparently called `bounded parametric polymorphism`.
}

fn using_trait_objects_that_allow_for_values_of_different_types() {
    pub trait Metrics {
        fn calculate_area(&mut self);
    }

    pub struct Shapes {
        //This is the new syntax with the `dyn` keyword which means dynamically dispatched.
        // Essentially this is the way of handling polymorphism in Rust. Essentially it is handling
        // polymorphism the same way as C++ does, by passing a pointer, its just wrapped up in a
        // pretty and more explicit way.
        pub shapes: Vec<Box<dyn Metrics>>,
    }

    impl Shapes {
        pub fn calc(&mut self) {
            for shapes in self.shapes.iter_mut() {
                shapes.calculate_area()
            }
        }
    }

    //Note that this is a similar way of implementing the same functionality. However, this forces
    // the vector to be of only a single type determined at compile time. Using the `dyn` keyword
    // above the vector will be able to hold multiple of different types that implement the
    // Metrics trait.
    pub struct AnotherShapes<T: Metrics> {
        pub shapes: Vec<T>,
    }

    impl<T> AnotherShapes<T>
        where
            T: Metrics {
        pub fn calc(&mut self) {
            for shapes in self.shapes.iter_mut() {
                shapes.calculate_area()
            }
        }
    }

    #[derive(Debug)]
    pub struct Triangle {
        base: isize,
        height: isize,
        area: isize,
    }

    impl Triangle {
        fn new(base: isize, height: isize) -> Triangle {
            Triangle{
                base,
                height,
                area: 0,
            }
        }
    }

    impl Metrics for Triangle {
        fn calculate_area(&mut self) {
            self.area = 1/2 * self.base * self.height;
        }
    }


    let mut shapes = AnotherShapes{
        shapes: Vec::from([Triangle::new(5, 4)])
    };

    shapes.calc();
    println!("another_shapes: {:?}", shapes.shapes[0]);


    let mut shapes = Shapes{
        //Note that unless the macro is used to create the vector, it must be case to the type of
        // `Box<dyn Metrics>`. Again this is just like C++. But the vector being able to
        // automatically handle it is also a nice change.
        shapes: Vec::from([Box::new(Triangle::new(5, 4)) as Box<dyn Metrics>]),
        // shapes: vec![Box::new(Triangle::new(5, 4))],
    };

    shapes.calc();
    println!("shapes: {:?}", shapes.shapes[0].calculate_area());


    //Note that there are two ways listed above of accomplishing the same thing. The first is the
    // class Shapes, this uses dynamic dispatch because the compiler cannot figure out the types.
    // The second is the class AnotherShapes, this uses static dispatch because the compiler can
    // figure out the type at compile time. Static dispatch allows the code to simply be replaced
    // at compile time with the specific types. This means that there is no runtime performance
    // penalty for using generics. However, dynamic dispatching must be figured out at runtime and
    // so these incur a performance penalty.
}
