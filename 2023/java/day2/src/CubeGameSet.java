import java.util.Map;
import java.util.HashMap;

public class CubeGameSet {
    private Map<String, Integer> cubes;
    public CubeGameSet(String input) {
        this.cubes = new HashMap<String, Integer>();
        String[] parts = input.split(", ");
        for (String part : parts) {
            String[] coloredCube = part.split(" ");
            try {
                this.cubes.put(coloredCube[1], Integer.valueOf(coloredCube[0]));
            } catch (NumberFormatException e) {
                System.out.println("Improper parse of colored cube amount: " + coloredCube[0] + " , " + coloredCube[1]);
                System.exit(1);
            }
        }
    }

    public Map<String, Integer> getCubes() {
        return this.cubes;
    }

    public boolean validate(int r, int g, int b) {
        if (this.cubes.containsKey("red")) {
            if (this.cubes.get("red") > r) {
                return false;
            }
        }
        if (this.cubes.containsKey("green")) {
            if (this.cubes.get("green") > g) {
                return false;
            }
        }
        if (this.cubes.containsKey("blue")) {
            if (this.cubes.get("blue") > b) {
                return false;
            }
        }
        return true;
    }
}
