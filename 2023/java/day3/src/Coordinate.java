import java.util.Objects;

public class Coordinate {
    private final int x;
    private final int y;

    public Coordinate(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() {
        return this.x;
    }

    public int getY() {
        return this.y;
    }

    @Override
    public boolean equals(Object o)
    {
        return this.x == ((Coordinate) o).x && this.y == ((Coordinate) o).y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}
