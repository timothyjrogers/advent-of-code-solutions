import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        partOne();
        partTwo();
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
        int sum = 0;
        for (String line : lines) {
            String payload = line.split(":\\s+")[1];
            Card card = new Card(payload.split("\\s+\\|\\s+")[0], payload.split("\\s+\\|\\s+")[1]);
            sum += card.calculateScore();
        }
        System.out.println("Part 1: " + sum);
    }

    private static void partTwo() {
        ArrayList<Card> cards = new ArrayList<>();
        HashMap<Integer, Integer> cardCounts = new HashMap<>();
        try {
            Path inputPath = Paths.get("input.txt");
            ArrayList<String> lines = (ArrayList<String>)Files.readAllLines(inputPath);
            for (int i = 0; i < lines.size(); i++) {
                String payload = lines.get(i).split(":\\s+")[1];
                cards.add(new Card(payload.split("\\s+\\|\\s+")[0], payload.split("\\s+\\|\\s+")[1]));
                cardCounts.put(i, 1);
            }
        } catch (IOException e) {
            System.out.println("Unable to open input file.");
            e.printStackTrace();
        }
        for (int i = 0; i < cards.size(); i++) {
            int winners = cards.get(i).countWinners();
            int numCards = cardCounts.get(i);
            for (int j = 1; j <= winners; j++) {
                cardCounts.put(i + j, cardCounts.get(i + j) + numCards);
            }
        }
        int sum = 0;
        for (Integer n : cardCounts.values()) {
            sum += n;
        }
        System.out.println("Part 2: " + sum);
    }
}