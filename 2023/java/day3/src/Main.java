import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        partOne();
        partTwo();
    }

    private static void partOne() {
        Schematic schematic = null;
        try {
            Path inputPath = Paths.get("input.txt");
            ArrayList<String> lines = (ArrayList<String>)Files.readAllLines(inputPath);
            schematic = new Schematic(lines);
        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        System.out.println("Part 1: " + schematic.calculatePartNumberSum());
    }
    private static void partTwo() {
        Schematic schematic = null;
        try {
            Path inputPath = Paths.get("input.txt");
            ArrayList<String> lines = (ArrayList<String>)Files.readAllLines(inputPath);
            schematic = new Schematic(lines);
        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        System.out.println("Part 2: " + schematic.calculateGearRatioSum());
    }
}