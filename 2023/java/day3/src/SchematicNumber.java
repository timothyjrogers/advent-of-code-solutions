import java.util.ArrayList;

public class SchematicNumber {
    private final int y;
    private final int xStart;
    private int xEnd;

    private int value;

    public SchematicNumber(int y, int x) {
        this.y = y;
        this.xStart = x;
        this.xEnd = -1;
        this.value = -1;
    }

    public void setEnd(int x) {
        this.xEnd = x;
    }

    public int getValue() {
        return this.value;
    }
    public void setValue(int value) {
        this.value = value;
    }

    public ArrayList<Coordinate> getNeighbors(int xMax, int yMax) {
        ArrayList<Coordinate> neighbors = new ArrayList<>();
        for (int x = this.xStart -1; x < this.xEnd + 2; x++) {
            if (this.y > 0) {
                if (x >= 0 && x <= xMax) {
                    neighbors.add(new Coordinate(x, this.y - 1));
                }
            }
            if (this.y < yMax) {
                if (x >= 0 && x <= xMax) {
                    neighbors.add(new Coordinate(x, this.y + 1));
                }
            }
        }
        if (this.xStart > 0) {
            neighbors.add(new Coordinate(this.xStart - 1, this.y));
        }
        if (this.xEnd < xMax) {
            neighbors.add(new Coordinate(this.xEnd + 1, this.y));
        }
        return neighbors;
    }
}
