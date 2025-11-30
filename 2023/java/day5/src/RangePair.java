public class RangePair {
    long sourceStart;
    long destinationStart;
    long length;

    public RangePair(long sourceStart, long destinationStart, long length) {
        this.sourceStart = sourceStart;
        this.destinationStart = destinationStart;
        this.length = length;
    }

    public boolean valueInRange(long value) {
        return value >= this.sourceStart && value < this.sourceStart + this.length;
    }

    public long mapValue(long value) {
        long offset = value - this.sourceStart;
        return this.destinationStart + offset;
    }
}
