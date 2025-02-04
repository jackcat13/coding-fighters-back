use rocket::serde::{Deserialize, Serialize};
use std::string::ToString;

use crate::resource::game_resource::QUESTION_SECONDS;

///GameProgressDto is used to interact with the game frontend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameProgressDto {
    pub game_id: String,
    pub current_question: i8,
    pub question_number: i8,
    pub question_content: QuestionDto,
}

///QuestionDto is used to interact with the game frontend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuestionDto {
    pub question_text: String,
    pub answer_1: String,
    pub answer_2: String,
    pub answer_3: String,
    pub answer_4: String,
    pub good_answer_number: i8,
    pub topic: String,
    pub remaing_time: u64,
}

pub fn questions_java() -> Vec<QuestionDto> {
    vec![
        QuestionDto {
            question_text: "In Java, what type is not a primitive data type ?".to_string(),
            answer_1: "long".to_string(),
            answer_2: "double".to_string(),
            answer_3: "String".to_string(),
            answer_4: "char".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what statement is true ?".to_string(),
            answer_1: "A class may implement multiple interfaces and may extend one class maximum"
                .to_string(),
            answer_2:
                "A class may implement multiple classes but is allowed to extend only one interface"
                    .to_string(),
            answer_3:
                "A class may implement multiple classes and may implement multiple interfaces"
                    .to_string(),
            answer_4: "A class must implement one interface and must extend one class".to_string(),
            good_answer_number: 1,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which of the below is valid way to instantiate an array in Java ?".to_string(),
            answer_1: "int myArray [] = {1, 3, 5};".to_string(),
            answer_2: "int myArray [] [] = {1,2,3,4};".to_string(),
            answer_3: "int [] myArray = (5, 4, 3);".to_string(),
            answer_4: "int [] myArray = {“1”, “2”, “3”};".to_string(),
            good_answer_number: 1,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what is the return type of the hashCode() method in the Object class ?".to_string(),
            answer_1: "Object".to_string(),
            answer_2: "int".to_string(),
            answer_3: "long".to_string(),
            answer_4: "void".to_string(),
            good_answer_number: 2,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what does the expression float a = 35 / 0 return ?".to_string(),
            answer_1: "0".to_string(),
            answer_2: "Not A Number".to_string(),
            answer_3: "Infinity".to_string(),
            answer_4: "RuntimeException".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Evaluate the following Java expression, if x=3, y=5, and z=10 : ++z + y - y + z + x++".to_string(),
            answer_1: "24".to_string(),
            answer_2: "23".to_string(),
            answer_3: "20".to_string(),
            answer_4: "25".to_string(),
            good_answer_number: 4,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the following tool is used to generate API documentation in HTML format from doc comments in source code ?".to_string(),
            answer_1: "javap tool".to_string(),
            answer_2: "javaw command".to_string(),
            answer_3: "Javadoc tool".to_string(),
            answer_4: "javah command".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the following creates a List of 3 visible items and multiple selections abled ?".to_string(),
            answer_1: "new List(false, 3)".to_string(),
            answer_2: "new List(3, true)".to_string(),
            answer_3: "new List(true, 3)".to_string(),
            answer_4: "new List(3, false)".to_string(),
            good_answer_number: 2,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the following for loop declaration is NOT valid ?".to_string(),
            answer_1: "for ( int i = 99; i >= 0; i / 9 )".to_string(),
            answer_2: "for ( int i = 7; i <= 77; i += 7 )".to_string(),
            answer_3: "for ( int i = 20; i >= 2; - -i )".to_string(),
            answer_4: "for ( int i = 2; i <= 20; i = 2* i )".to_string(),
            good_answer_number: 1,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which method of the Class.class is used to determine the name of a class represented by the class object as a String ?".to_string(),
            answer_1: "getClass()".to_string(),
            answer_2: "intern()".to_string(),
            answer_3: "getName()".to_string(),
            answer_4: "toString()".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, in which process, a local variable has the same name as one of the instance variables ?".to_string(),
            answer_1: "It has only methods".to_string(),
            answer_2: "Objects can't be created".to_string(),
            answer_3: "It has a fixed class name".to_string(),
            answer_4: "It has no class name".to_string(),
            good_answer_number: 4,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which package contains the Random class ?".to_string(),
            answer_1: "java.util package".to_string(),
            answer_2: "java.lang package".to_string(),
            answer_3: "java.awt package".to_string(),
            answer_4: "java.io package".to_string(),
            good_answer_number: 1,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what do you mean by nameless objects ?".to_string(),
            answer_1: "An object created by using the new keyword".to_string(),
            answer_2: "An object of a superclass created in the subclass".to_string(),
            answer_3: "An object without having any name but having a reference".to_string(),
            answer_4: "An object that has no reference".to_string(),
            good_answer_number: 4,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, an interface with no fields or methods is known as a ______".to_string(),
            answer_1: "Runnable interface".to_string(),
            answer_2: "Marker interface".to_string(),
            answer_3: "Abstract interface".to_string(),
            answer_4: "CharSequence interface".to_string(),
            good_answer_number: 2,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the following is an immediate subclass of the Panel class?".to_string(),
            answer_1: "Applet class".to_string(),
            answer_2: "Window class".to_string(),
            answer_3: "Frame class".to_string(),
            answer_4: "Dialog class".to_string(),
            good_answer_number: 1,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which option is false about the final keyword ?".to_string(),
            answer_1: "A final method cannot be overridden in its subclasses.".to_string(),
            answer_2: "A final class cannot be extended.".to_string(),
            answer_3: "A final class cannot extend other classes.".to_string(),
            answer_4: "A final method can be inherited.".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of these classes are the direct subclasses of the Throwable class ?".to_string(),
            answer_1: "RuntimeException and Error class".to_string(),
            answer_2: "Exception and VirtualMachineError class".to_string(),
            answer_3: "Error and Exception class".to_string(),
            answer_4: "IOException and VirtualMachineError class".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What do you mean by chained exceptions in Java ?".to_string(),
            answer_1: "Exceptions occurred by the VirtualMachineError".to_string(),
            answer_2: "An exception caused by other exceptions".to_string(),
            answer_3: "Exceptions occur in chains with discarding the debugging information".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 2,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, in which memory a String is stored, when we create a string using new operator ?".to_string(),
            answer_1: "Stack".to_string(),
            answer_2: "String memory".to_string(),
            answer_3: "Heap memory".to_string(),
            answer_4: "Random storage space".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what is the use of the intern() method ?".to_string(),
            answer_1: "It returns the existing string from memory".to_string(),
            answer_2: "It creates a new string in the database".to_string(),
            answer_3: "It modifies the existing string in the database".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 1,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the following is a marker interface ?".to_string(),
            answer_1: "Runnable interface".to_string(),
            answer_2: "Remote interface".to_string(),
            answer_3: "Readable interface".to_string(),
            answer_4: "Result interface".to_string(),
            good_answer_number: 2,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which of the following is a reserved keyword in Java ?".to_string(),
            answer_1: "object".to_string(),
            answer_2: "strictfp".to_string(),
            answer_3: "main".to_string(),
            answer_4: "system".to_string(),
            good_answer_number: 2,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which keyword is used for accessing the features of a package ?".to_string(),
            answer_1: "package".to_string(),
            answer_2: "import".to_string(),
            answer_3: "extends".to_string(),
            answer_4: "export".to_string(),
            good_answer_number: 2,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In java, jar stands for_____".to_string(),
            answer_1: "Java Archive Runner".to_string(),
            answer_2: "Java Application Resource".to_string(),
            answer_3: "Java Application Runner".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 4,
            topic: "Java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "in java, which of the following is false ?".to_string(),
            answer_1: "the rt.jar stands for the runtime jar".to_string(),
            answer_2: "it is an optional jar file".to_string(),
            answer_3: "it contains all the compiled class files".to_string(),
            answer_4: "all the classes available in rt.jar is known to the jvm".to_string(),
            good_answer_number: 2,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what is the use of \\w in regex ?".to_string(),
            answer_1: "Used for a whitespace character".to_string(),
            answer_2: "Used for a non-whitespace character".to_string(),
            answer_3: "Used for a word character".to_string(),
            answer_4: "Used for a non-word character".to_string(),
            good_answer_number: 3,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the given methods are of Object class ?".to_string(),
            answer_1: "notify(), wait( long msecs ), and synchronized()".to_string(),
            answer_2: "wait( long msecs ), interrupt(), and notifyAll()".to_string(),
            answer_3: "notify(), notifyAll(), and wait()".to_string(),
            answer_4: "sleep( long msecs ), wait(), and notify()".to_string(),
            good_answer_number: 3,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the following is a valid syntax to synchronize the HashMap ?".to_string(),
            answer_1: "Map m = hashMap.synchronizeMap();".to_string(),
            answer_2: "HashMap map = hashMap.synchronizeMap();".to_string(),
            answer_3: "Map m1 = Collections.synchronizedMap(hashMap);".to_string(),
            answer_4: "Map m2 = Collection.synchronizeMap(hashMap);".to_string(),
            good_answer_number: 3,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what is meant by the classes and objects that dependents on each other ?".to_string(),
            answer_1: "Tight Coupling".to_string(),
            answer_2: "Cohesion".to_string(),
            answer_3: "Loose Coupling".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 1,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Javan how many threads can be executed at a time ?".to_string(),
            answer_1: "Only one thread".to_string(),
            answer_2: "Multiple threads".to_string(),
            answer_3: "Only main (main() method) thread".to_string(),
            answer_4: "Two threads".to_string(),
            good_answer_number: 2,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, if three threads trying to share a single object at the same time, which condition will arise in this scenario ?".to_string(),
            answer_1: "Time-Lapse".to_string(),
            answer_2: "Critical situation".to_string(),
            answer_3: "Race condition".to_string(),
            answer_4: "Recursion".to_string(),
            good_answer_number: 3,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, if a thread goes to sleep".to_string(),
            answer_1: "It releases all the locks it has.".to_string(),
            answer_2: "It does not release any locks.".to_string(),
            answer_3: "It releases half of its locks.".to_string(),
            answer_4: "It releases all of its lock except one.".to_string(),
            good_answer_number: 2,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, which of the following modifiers can be used for a variable so that it can be accessed by any thread or a part of a program ?".to_string(),
            answer_1: "global".to_string(),
            answer_2: "transient".to_string(),
            answer_3: "volatile".to_string(),
            answer_4: "default".to_string(),
            good_answer_number: 3,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, in character stream I/O, a single read/write operation performs _____".to_string(),
            answer_1: "Two bytes read/write at a time.".to_string(),
            answer_2: "Eight bytes read/write at a time.".to_string(),
            answer_3: "One byte read/write at a time.".to_string(),
            answer_4: "Five bytes read/ write at a time.".to_string(),
            good_answer_number: 1,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Java, what is the default encoding for an OutputStreamWriter ?".to_string(),
            answer_1: "UTF-8".to_string(),
            answer_2: "Default encoding of the host platform".to_string(),
            answer_3: "UTF-12".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 2,
            topic: "java".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
    ]
}

pub fn questions_rust() -> Vec<QuestionDto> {
    vec![QuestionDto {
        question_text: "What is a closure in Rust ?".to_string(),
        answer_1: "A function that is declared in a scope and captures its environment".to_string(),
        answer_2: "An anonymous function that doesn't capture its environment".to_string(),
        answer_3: "A function declared in a scope but doesn't capture its environment".to_string(),
        answer_4: "An anonymous function that captures its environment".to_string(),
        good_answer_number: 4,
        topic: "Rust".to_string(),
        remaing_time: QUESTION_SECONDS,
    }]
}

pub fn questions_kotlin() -> Vec<QuestionDto> {
    vec![
        QuestionDto {
            question_text: "What is Kotlin ?".to_string(),
            answer_1: "A new version of Java".to_string(),
            answer_2: "A JavaScript framework".to_string(),
            answer_3: "A statically-typed programming language for the JVM, Android and browser".to_string(),
            answer_4: "A database management system".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which platform does Kotlin primarily target ?".to_string(),
            answer_1: "Python Bytecode".to_string(),
            answer_2: "JavaScript".to_string(),
            answer_3: "JVM Bytecode".to_string(),
            answer_4: "PHP".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Are semicolons (;) mandatory at the end of code statements in Kotlin ?".to_string(),
            answer_1: "True".to_string(),
            answer_2: "False".to_string(),
            answer_3: "".to_string(),
            answer_4: "".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What paradigm(s) does the Kotlin programming language follow ?".to_string(),
            answer_1: "Only Object-Oriented".to_string(),
            answer_2: "Procedural".to_string(),
            answer_3: "Only Functional".to_string(),
            answer_4: "Both Object-Oriented and Functional".to_string(),
            good_answer_number: 4,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you declare a variable in Kotlin?".to_string(),
            answer_1: "let myVariable = 10;".to_string(),
            answer_2: "let myVariable: Int = 10".to_string(),
            answer_3: "const myVariable = 10;".to_string(),
            answer_4: "var myVariable: Int = 10".to_string(),
            good_answer_number: 4,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you define a variable in Kotlin that cannot be reassigned ?".to_string(),
            answer_1: "var".to_string(),
            answer_2: "val".to_string(),
            answer_3: "const;".to_string(),
            answer_4: "final".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you declare a nullable variable in Kotlin ?".to_string(),
            answer_1: "var name: String?".to_string(),
            answer_2: "var name: String".to_string(),
            answer_3: "var name: String = null".to_string(),
            answer_4: "String name = null".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What is the difference between val and var in Kotlin ?".to_string(),
            answer_1: "They are identical and can be used interchangeably".to_string(),
            answer_2: "'val' declares mutable variables, and 'var' declares immutable ones".to_string(),
            answer_3: "'val' declares immutable variables, and 'var' declares mutable ones".to_string(),
            answer_4: "'val' is used for local variables, and 'var' is used for globak variables".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you define a function in Kotlin ?".to_string(),
            answer_1: "fun myFunction() {}".to_string(),
            answer_2: "def myFunction() {}".to_string(),
            answer_3: "function myFunction() {}".to_string(),
            answer_4: "fun = myFunction() {}".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Why is the when expression used in Kotlin ?".to_string(),
            answer_1: "To create a loop".to_string(),
            answer_2: "To define a switch-case statement".to_string(),
            answer_3: "To define a conditional statement".to_string(),
            answer_4: "To create a lambda function".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What does ?. operator do in Kotlin ?".to_string(),
            answer_1: "Null-safe type casting".to_string(),
            answer_2: "Null-sage function calling".to_string(),
            answer_3: "Null-sage member access".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What is the default visibility modifiers in Kotlin if no modifier is specified ?".to_string(),
            answer_1: "public".to_string(),
            answer_2: "private".to_string(),
            answer_3: "internal".to_string(),
            answer_4: "protected".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which keyword is used to create a singleton in Kotlin ?".to_string(),
            answer_1: "static".to_string(),
            answer_2: "singleton".to_string(),
            answer_3: "single".to_string(),
            answer_4: "object".to_string(),
            good_answer_number: 4,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What is the main purpose of the let function in Kotlin ?".to_string(),
            answer_1: "To facilitate null checks".to_string(),
            answer_2: "To execute a block of code and return a result".to_string(),
            answer_3: "To transform an object".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which feature in Kotlin helps to prevent NullPointerExceptions ?".to_string(),
            answer_1: "Safe call operator (?.)".to_string(),
            answer_2: "Non-null Assertion Operator (!!)".to_string(),
            answer_3: "Elvis operator (?:)".to_string(),
            answer_4: "Safe cast operator (as?)".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What is the purpose of the open modifier in Kotlin ?".to_string(),
            answer_1: "It allows a class to be instantiated".to_string(),
            answer_2: "It makes a function available for overriding".to_string(),
            answer_3: "It enforces strict typing for variables".to_string(),
            answer_4: "It allows a function to be called only from within its own class".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you create a single-line comment in Kotlin ?".to_string(),
            answer_1: "// This is a comment".to_string(),
            answer_2: "/* This is a comment */".to_string(),
            answer_3: "-- This is a comment".to_string(),
            answer_4: "# This is a comment".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How can we write a multi-line comment in Kotlin ?".to_string(),
            answer_1: "// This is a comment".to_string(),
            answer_2: "/* This is a comment */".to_string(),
            answer_3: "-- This is a comment".to_string(),
            answer_4: "# This is a comment".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What is the role of the init block in Kotlin ?".to_string(),
            answer_1: "To initialize the superclass".to_string(),
            answer_2: "To initialize an object after the constructor has been called".to_string(),
            answer_3: "To initialize static variables".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you call a function in Kotlin ?".to_string(),
            answer_1: "functionName()".to_string(),
            answer_2: "call functionName".to_string(),
            answer_3: "Function -> functionName".to_string(),
            answer_4: "functionName:call".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which Kotlin construct allows a block of code to be executed a specific number of times ?".to_string(),
            answer_1: "for".to_string(),
            answer_2: "while".to_string(),
            answer_3: "repeat".to_string(),
            answer_4: "loop".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which keyword are used to handle conditional statements in Kotlin ?".to_string(),
            answer_1: "if".to_string(),
            answer_2: "when".to_string(),
            answer_3: "both a and b".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What is the correct way to define a primary constructor in Kotlin ?".to_string(),
            answer_1: "constructor()".to_string(),
            answer_2: "class constructor()".to_string(),
            answer_3: "primary constructor()".to_string(),
            answer_4: "class Person()".to_string(),
            good_answer_number: 4,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Kotlin, what is the main purpose of the return keyword ?".to_string(),
            answer_1: "To declare a function".to_string(),
            answer_2: "To create a loop".to_string(),
            answer_3: "To terminate a function and return a value".to_string(),
            answer_4: "To define a class".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What is the primary use of the 'with' function in Kotlin ?".to_string(),
            answer_1: "To create an extension function".to_string(),
            answer_2: "To apply multiple transformations to a collection".to_string(),
            answer_3: "To establish a scope in which an object's properties and functions can be accessed directly without specifying the object's name".to_string(),
            answer_4: "To create an anonymous function".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What does the 'also' function do in Kotlin ?".to_string(),
            answer_1: "It's a scoping function that also executes a block of code".to_string(),
            answer_2: "It runs a block of code and returns the object it was called on".to_string(),
            answer_3: "It runs a block of code and returns the result".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which of these is not a loop structure in Kotlin ?".to_string(),
            answer_1: "for loop".to_string(),
            answer_2: "while loop".to_string(),
            answer_3: "until loop".to_string(),
            answer_4: "do-while loop".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do we throw an exception in Kotlin ?".to_string(),
            answer_1: "throw Exception()".to_string(),
            answer_2: "raise Exception()".to_string(),
            answer_3: "Exception.throw()".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What methods can be used to achieve abstraction in Kotlin ?".to_string(),
            answer_1: "Through abstract classes only ?".to_string(),
            answer_2: "Through interfaces only".to_string(),
            answer_3: "Through both abstract classes and interfaces".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you declare an array of integers in Kotlin ?".to_string(),
            answer_1: "val numbers = arrayOf(1, 2, 3)".to_string(),
            answer_2: "val numbers = listOf(1, 2, 3)".to_string(),
            answer_3: "val numbers = [1, 2, 3]".to_string(),
            answer_4: "val numbers = Array(3) {0, 1, 2}".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which of the following is not a basic type in Kotlin ?".to_string(),
            answer_1: "Boolean".to_string(),
            answer_2: "String".to_string(),
            answer_3: "Float".to_string(),
            answer_4: "Char".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you declare a String in Kotlin ?".to_string(),
            answer_1: "val str: String = \"Hello World\"".to_string(),
            answer_2: "String str = \"Hello World\"".to_string(),
            answer_3: "val str = String(\"Hello World\")".to_string(),
            answer_4: "String str = new String(\"Hello World\")".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Kotlin, how do you compare two Strings for equality ?".to_string(),
            answer_1: "str1==srt2".to_string(),
            answer_2: "str1.equals(str2)".to_string(),
            answer_3: "Both a and b".to_string(),
            answer_4: "str1.sameAs(str2)".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which property can be used to find the length of a string ?".to_string(),
            answer_1: "size".to_string(),
            answer_2: "length".to_string(),
            answer_3: "count".to_string(),
            answer_4: "charCount".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "In Kotlin, which collection type has an order and can contain duplicate elements ?".to_string(),
            answer_1: "Set".to_string(),
            answer_2: "List".to_string(),
            answer_3: "Map".to_string(),
            answer_4: "All of the above".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which function is used to iterate over a collection in Kotlin ?".to_string(),
            answer_1: "forEach()".to_string(),
            answer_2: "for()".to_string(),
            answer_3: "map()".to_string(),
            answer_4: "filter()".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which of these functions can transform a list in Kotlin ?".to_string(),
            answer_1: "map()".to_string(),
            answer_2: "filter()".to_string(),
            answer_3: "forEach()".to_string(),
            answer_4: "None of the above".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "Which collection type ensures element uniqueness in Kotlin ?".to_string(),
            answer_1: "List".to_string(),
            answer_2: "Set".to_string(),
            answer_3: "Map".to_string(),
            answer_4: "MutableList".to_string(),
            good_answer_number: 2,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "How do you create an empty list in Kotlin ?".to_string(),
            answer_1: "emptyList()".to_string(),
            answer_2: "listOf()".to_string(),
            answer_3: "list()".to_string(),
            answer_4: "mutableListOf()".to_string(),
            good_answer_number: 1,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },
        QuestionDto {
            question_text: "What does the mapOf() function do in Kotlin ?".to_string(),
            answer_1: "It creates a new List".to_string(),
            answer_2: "It creates a new Set".to_string(),
            answer_3: "It creates a new Map".to_string(),
            answer_4: "It creates a new Queue".to_string(),
            good_answer_number: 3,
            topic: "Kotlin".to_string(),
            remaing_time: QUESTION_SECONDS,
        },

    ]
}
