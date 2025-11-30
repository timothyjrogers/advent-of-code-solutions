import java.util.ArrayList;
import java.util.Map;

public class CubeGame {

    private ArrayList<CubeGameSet> cubeSets;
    public CubeGame(String input) {
        this.cubeSets = new ArrayList<CubeGameSet>();
        String[] parts = input.split(": ")[1].split("; ");
        for (String part : parts) {
            cubeSets.add(new CubeGameSet(part));
        }
    }
    public boolean validGame(int r, int g, int b) {
        for (CubeGameSet cubeSet : this.cubeSets) {
            if (!cubeSet.validate(r, g, b)) {
                return false;
            }
        }
        return true;
    }

    public int calculatePower() {
        int rMax = 0;
        int gMax = 0;
        int bMax = 0;
        for (CubeGameSet cubeSet : this.cubeSets) {
            Map<String, Integer> cubes = cubeSet.getCubes();
            if (cubes.getOrDefault("red", 0) > rMax) {
                rMax = cubes.getOrDefault("red", 0);
            }
            if (cubes.getOrDefault("green", 0) > gMax) {
                gMax = cubes.getOrDefault("green", 0);
            }
            if (cubes.getOrDefault("blue", 0) > bMax) {
                bMax = cubes.getOrDefault("blue", 0);
            }
        }
        return rMax * gMax * bMax;
    }
}