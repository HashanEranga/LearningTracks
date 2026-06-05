var circle = new Circle(7);
Console.WriteLine($"Area of the circle : {Area(circle)}");

var threpisium = new Threpisium(10, 5);
Console.WriteLine($"Area of the threpisium : {Area(threpisium)}");

Shape[] shapes = [new Circle(7), new Circle(0), new Square(80), new Rectangle(4, 4),
    threpisium];
foreach (var s in shapes)
    Console.WriteLine($"{s.GetType().Name}: {Classify(s)}");


var drawing = new Drawing()
{
    Name = "My drawing",
    Shapes = shapes
};

Console.WriteLine(ShapeFormatter.Describe(drawing.Shapes[0]));

Console.WriteLine(FindLargest(drawing.Shapes));



static double Area(Shape shape) => shape switch
{
    Circle c => Math.PI * c.Radius * c.Radius,
    Square s => s.Side * s.Side,
    Rectangle r => r.Width * r.Height,
    Threpisium t => t.BaseLine * t.PerpHeight
};

static string Classify(Shape shape) => shape switch
{
    Circle { Radius: <= 0 } => "invalid circle",
    Circle { Radius: > 100 } => "huge circle",
    Rectangle r when Math.Abs(r.Width - r.Height) <= float.Epsilon => "secretly a square",
    Square { Side: > 50} => "big square",
    _ => "ordinary"
};

static Shape? FindLargest(IReadOnlyList<Shape> shapes)
{
    Shape? largest = null;
    double maxArea = double.NegativeInfinity;
    foreach (var s in shapes)
    {
        var area = Area(s);
        if (area > maxArea)
        {
            maxArea = area;
            largest = s;
        }
    }

    return largest;
};

file static class ShapeFormatter
{
    public static string Describe(Shape s) => $"{s.GetType().Name} (area {ShapeMath.Area(s):F2})";
}

file static class ShapeMath
{
    public static double Area(Shape shape) => shape switch
    {
        Circle c => Math.PI * c.Radius * c.Radius,
        Square s => s.Side * s.Side,
        Rectangle r => r.Width * r.Height,
        Threpisium t => t.BaseLine * t.PerpHeight,
        _ => throw new NotImplementedException()
    };
}

public record Drawing
{
    public required string Name { get; init; }
    public required IReadOnlyList<Shape> Shapes { get; init; }
} 

public abstract record Shape;
public sealed record Circle(double Radius) : Shape;
public sealed record Rectangle(double Width, double Height) : Shape;
public sealed record Square(double Side) : Shape;
public sealed record Threpisium(double BaseLine, double PerpHeight) : Shape;




