// ErrorTest.java
package com.test.error;

import java.util.*;
import java.io.File;
import java.time.LocalDate;
import java.util.Date; // Duplicate import with different class

public class ErrorTest extends ArrayList<String> implements Runnable, Serializable {

	private static final String MESSAGE = "Hello World" // Missing semicolon

	String name = "test" // Missing semicolon

	int uninitialized; // Unused variable

	// Wrong main method signature
	public void main(String args) {
		System.out.println("Wrong main method");
	}

	public static void main(String[] args) {
        System.out.println("Starting compile error chaos...");

        int x = 10;
        String y = x;                                    // Incompatible types

        List<String> list = new ArrayList();
        list.add(123);                                   // Cannot add int to List<String>

        undefinedMethod();                               // Cannot find symbol

        String[] arr = new String[5];
        System.out.println(arr[10]);                     // Bad array access

        for (int i = 0 i < 10 i++) {                     // Missing semicolons
            System.out.println(i)
        }

        if (x = 5) {                                     // Assignment instead of ==
            System.out.println("Assignment in if");
        }

        SomeNonExistentClass obj = new SomeNonExistentClass(); // Class not found

        Date date = new Date();
        LocalDate ld = LocalDate.now();

        checkAge(-5);
    }

	// Missing return type
	checkAge(int age) {
        if (age < 0)
            throw new IllegalArgumentException();
        return "Valid Age";
    }

	// Syntax error in method declaration
	public static int divide
	int a,
	int b)
	{ // Missing parentheses
		return a / b;
	}

	// Duplicate methods with different return types (not allowed)
	public void doSomething() {
	}

	public int doSomething() {
		return 42;
	}

	@Override
	public void run() {
		System.out.println("Running...");
	}

	// Missing closing parenthesis
	public void brokenMethod( {
        System.out.println("Missing parenthesis"
    }

	// Invalid characters in method name
	public void test#InvalidMethod() {
        int @number = 10;
    }

	// Abstract method not implemented properly
	public void missingOverrideMethod() {
	}
}
