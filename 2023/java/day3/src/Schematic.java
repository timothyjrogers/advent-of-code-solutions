import java.util.ArrayList;
import java.util.HashMap;

public class Schematic {
    private final ArrayList<String> schematic;
    private final ArrayList<SchematicNumber> partNumbers;

    public Schematic(ArrayList<String> input) {
        this.schematic = input;
        this.partNumbers = new ArrayList<>();
        this.findNumbers();
    }

    private void findNumbers() {
        for (int y = 0; y < this.schematic.size(); y++) {
            String cur = this.schematic.get(y);
            int xStart = -1;
            int xEnd = -1;
            SchematicNumber curNumber = null;
            for (int x = 0; x < cur.length(); x++) {
                if (Character.isDigit(cur.charAt(x))) {
                    if (curNumber == null) {
                        xStart = x;
                        curNumber = new SchematicNumber(y, xStart);
                    }
                    xEnd = x;
                } else if ((!Character.isDigit(cur.charAt(x)) && curNumber != null)) {
                    curNumber.setEnd(xEnd);
                    try {
                        curNumber.setValue(Integer.valueOf(cur.substring(xStart, xEnd + 1)));
                    } catch (NumberFormatException e) {
                        e.printStackTrace();
                        System.exit(1);
                    }
                    this.partNumbers.add(curNumber);
                    xStart = -1;
                    xEnd = -1;
                    curNumber = null;
                }
            }
            if (curNumber != null) {
                curNumber.setEnd(xEnd);
                curNumber.setValue(Integer.valueOf(cur.substring(xStart, xEnd + 1)));
                this.partNumbers.add(curNumber);
            }
        }
    }
    public int calculatePartNumberSum() {
        int sum = 0;
        for (SchematicNumber current : this.partNumbers) {
            for (Coordinate neighbor : current.getNeighbors(this.schematic.get(0).length() - 1, this.schematic.size() - 1)) {
                Character currentNeighbor = this.schematic.get(neighbor.getY()).charAt(neighbor.getX());
                if (currentNeighbor != '.') {
                    sum += current.getValue();
                    break;
                }
            }
        }
        return sum;
    }

    public int calculateGearRatioSum() {
        int sum = 0;
        HashMap<Coordinate, ArrayList<Integer>> gears = new HashMap<>();
        for (SchematicNumber current : this.partNumbers) {
            for (Coordinate neighbor : current.getNeighbors(this.schematic.get(0).length() - 1, this.schematic.size() - 1)) {
                Character currentNeighbor = this.schematic.get(neighbor.getY()).charAt(neighbor.getX());
                if (currentNeighbor == '*') {
                    Coordinate gearCoordinate = new Coordinate(neighbor.getX(), neighbor.getY());
                    if (gears.containsKey(gearCoordinate)) {
                        gears.get(gearCoordinate).add(current.getValue());
                    } else {
                        gears.put(gearCoordinate, new ArrayList<>());
                        gears.get(gearCoordinate).add(current.getValue());
                    }
                }
            }
        }
        for (ArrayList<Integer> numbers : gears.values()) {
            if (numbers.size() == 2) {
                sum += numbers.get(0) * numbers.get(1);
            }
        }
        return sum;
    }

}
