use position::Position;

enum ContinuityType {
    Position,
    Tangent,
    Curvature,
}

/// Maybe have another enum that can describe a positional constraint vs a hint of location?
enum PointConstraint {
    Floating(Position<3>),
    Fixed(Position<3>),
    CoincidentWithPoint(Point),
    CoincidentWithLine(Line),
    CoincidentWithPlane(Plane),
    ProjectedPoint(Point, Plane),
}

/// For most of these it should probably be using Curves and not Lines
/// And maybe this should be for Curves and not lines?
enum LineConstraint {
    Fixed(PointConstraint, PointConstraint),
    PerpendicularToLine(Line),
    ParallelToLine(Line),
    PerpendicularToPlane(Plane),
    ColinearToLine(Line),
    OnPlane(Plane),
}

/// Tangency and perpendicularity constraints are the only ones that really need to be implimented for now
/// THe rest are just to lay out what I think is needed over all
enum LineAtPointConstraint {
    TangentToLine(Line, Point, Line),
    TangentToPlane(Line, Point, Plane),
    PerpendicularToLine(Line, Point, Line),
    PerpendicularToPlane(Line, Point, Plane),
    ContinuityToLine(Line, Point, Line, ContinuityType),
}

/// These are just a placeholder rn, I think the nurbis should be handled
enum PlaneAtEdgeConstraint {
    CoincidentWithLine(Plane, Line, Line),
    ContinuityToPlane(Plane, Line, Plane, ContinuityType),
}
