import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.*;

public class Card {
    List<Integer> winningNumbers;
    List<Integer> playerNumbers;

    public Card(String winningNumbers, String playerNumbers) {
        this.winningNumbers = Arrays.stream(winningNumbers.split("\\s+"))
                .map(Integer::valueOf)
                .collect(Collectors.toList());
        this.playerNumbers = Arrays.stream(playerNumbers.split("\\s+"))
                .map(Integer::valueOf)
                .collect(Collectors.toList());
    }
    public int countWinners() {
        int winners = 0;
        for (Integer n : this.playerNumbers) {
            if (this.winningNumbers.contains(n)) {
                winners++;
            }
        }
        return winners;
    }
    public int calculateScore() {
        return (int)Math.pow(2, this.countWinners() - 1);
    }
}
