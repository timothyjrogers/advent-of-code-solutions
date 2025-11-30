import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        partOne();
        partTwo();
    }

    private static void partOne() {
        int result = 0;
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader("input.txt"));
            String line = reader.readLine();
            while (line != null) {
                int first = -1;
                int last = -1;
                for (int index = 0; index < line.length(); index++) {
                    if (Character.isDigit(line.charAt(index))) {
                        if (first < 0){
                            first = line.charAt(index) - '0';
                        }
                        last = line.charAt(index) - '0';
                    }
                }
                result = result + (first * 10) + last;
                line = reader.readLine();
            }

        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        System.out.println("Part 1: " + result);
    }

    private static void partTwo() {
        int result = 0;
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader("input.txt"));
            String line = reader.readLine();
            while (line != null) {
                line = line.replaceAll("one", "on1e");
                line = line.replaceAll("two", "tw2o");
                line = line.replaceAll("three", "thre3e");
                line = line.replaceAll("four", "fou4r");
                line = line.replaceAll("five", "fiv5e");
                line = line.replaceAll("six", "si6x");
                line = line.replaceAll("seven", "seve7n");
                line = line.replaceAll("eight", "eigh8t");
                line = line.replaceAll("nine", "nin9e");
                line = line.replaceAll("zero", "zer0o");
                int first = -1;
                int last = -1;
                for (int index = 0; index < line.length(); index++) {
                    if (Character.isDigit(line.charAt(index))) {
                        if (first < 0){
                            first = line.charAt(index) - '0';
                        }
                        last = line.charAt(index) - '0';
                    }
                }
                result = result + (first * 10) + last;
                line = reader.readLine();
            }

        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        System.out.println("Part 2: " + result);
    }
}