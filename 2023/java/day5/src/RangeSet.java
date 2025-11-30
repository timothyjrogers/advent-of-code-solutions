import java.util.ArrayList;
import java.util.List;

public class RangeSet {
    List<Range> pairs;

    public RangeSet() {
        this.pairs = new ArrayList<>();
    }

    public void addRange(Range range) {
        this.pairs.add(range);
    }

    public void mergeRanges(Range source, Range destination) {
        //Find matches in this.pairs for the source range, and convert them to the destination
        //Unmatched ranges or parts of ranges should be omitted
        
    }
}
