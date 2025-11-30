import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) {
        partOne();
        //partTwo();
    }

    private static void partOne() {
        ArrayList<String> lines = null;
        try {
            Path inputPath = Paths.get("input.txt");
            lines = (ArrayList<String>)Files.readAllLines(inputPath);
        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        List<Long> seeds = Arrays.stream(lines.get(0).split(": ")[1].split(" "))
                .map(Long::valueOf)
                .collect(Collectors.toList());
        lines.remove(0);

        List<List<RangePair>> ranges = new ArrayList<>();
        String mapName = null;
        for (String line : lines) {
            if (line.length() == 0) {
                mapName = null;
            } else if (mapName != null) {
                List<Long> parts = Arrays.stream(line.split(" "))
                        .map(Long::valueOf)
                        .collect(Collectors.toList());
                ranges.get(ranges.size() - 1).add(new RangePair(parts.get(1), parts.get(0), parts.get(2)));
            } else {
                mapName = line.split(" ")[0];
                ranges.add(new ArrayList<RangePair>());
            }
        }
        long minLocation = Long.MAX_VALUE;
        for (long seed : seeds) {
            long value = seed;
            for (int i = 0; i < ranges.size(); i++) {
                for (RangePair pair : ranges.get(i)) {
                    if (pair.valueInRange(value)) {
                        value = pair.mapValue(value);
                        break;
                    }
                }
            }
            if (value < minLocation) {
                minLocation = value;
            }
        }
        System.out.println("Part 1: " + minLocation);
    }

    private static void partTwo() {
        ArrayList<String> lines = null;
        try {
            Path inputPath = Paths.get("input.txt");
            lines = (ArrayList<String>)Files.readAllLines(inputPath);
        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        RangeSet ranges = new RangeSet();
        List<Long> seeds = Arrays.stream(lines.get(0).split(": ")[1].split(" "))
                .map(Long::valueOf)
                .collect(Collectors.toList());
        lines.remove(0);
        for (int i = 0; i < seeds.size(); i += 2) {
            ranges.addRange(new Range(seeds.get(i), seeds.get(i+1)));
        }
        //TODO - Merge next set of ranges into current ranges
        String mapName = null;
        for (String line : lines) {
            if (line.length() == 0) {
                mapName = null; //Reset "in map" condition on blank line
            } else if (mapName != null) {
                List<Long> parts = Arrays.stream(line.split(" "))
                        .map(Long::valueOf)
                        .collect(Collectors.toList());
                Range source = new Range(parts.get(1), parts.get(1) + parts.get(2));
                Range destination = new Range(parts.get(0), parts.get(0) + parts.get(2));
                ranges.mergeRanges(source, destination);
            } else {
                //TODO - Starting new set of ranges (e.g. seeds-to-soil map:)
                mapName = line.split(" ")[0];
            }
        }
        System.out.println("Part 2: " + 0);
    }
}