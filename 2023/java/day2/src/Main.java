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
        int id = 1;
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader("input.txt"));
            String line = reader.readLine();
            while (line != null) {
                CubeGame cur = new CubeGame(line);
                if (cur.validGame(12, 13, 14)) {
                    result += id;
                }
                id++;
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
        int id = 1;
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader("input.txt"));
            String line = reader.readLine();
            while (line != null) {
                CubeGame cur = new CubeGame(line);
                result += cur.calculatePower();
                line = reader.readLine();
            }

        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        System.out.println("Part 2: " + result);
    }
}