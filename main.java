
import java.util.stream.IntStream;

class main{
	public static void main(String[]args){
		var a = 10;
		var c = 'f';
		System.out.println("hello world");
		IntStream.range(0, a).map((i) ->(i+1)).forEach(System.out::println);
		System.out.println(c);
		// IntStream.range(int('a'), int(c) + 1);
	}
}
