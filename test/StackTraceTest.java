public class StackTraceTest {
    
    public static void main(String[] args) {
        System.out.println("Starting stack trace test...");
        
        try {
            causeException();
        } catch (Exception e) {
            System.err.println("=== EXCEPTION CAUGHT ===");
            e.printStackTrace();  // This prints the full stack trace
        }
        
        System.out.println("Program finished.");
    }
    
    private static void causeException() {
        // Method 1: Deep call stack for better testing
        level1();
    }
    
    private static void level1() {
        level2();
    }
    
    private static void level2() {
        level3();
    }
    
    private static void level3() {
        // Multiple common exception types - uncomment one at a time
        
        // 1. NullPointerException (most common)
        String str = null;
        System.out.println(str.length());
        
        // 2. ArrayIndexOutOfBoundsException
        // int[] arr = new int[5];
        // System.out.println(arr[10]);
        
        // 3. ArithmeticException
        // int x = 10 / 0;
        
        // 4. ClassCastException
        // Object obj = "Hello";
        // Integer num = (Integer) obj;
        
        // 5. IllegalArgumentException with cause
        // throw new IllegalArgumentException("Invalid input", new NullPointerException("Root cause"));
    }
}
