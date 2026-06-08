using UnityEngine;

public class HighwayRoadReferences : MonoBehaviour
{
    [SerializeField] private Transform laneCenter;
    [SerializeField] private Transform leftLaneBound;
    [SerializeField] private Transform rightLaneBound;
    [SerializeField] private Transform carSpawnPoint;
    [SerializeField] private float laneWidth = 4f;
    [SerializeField] private float roadLength = 120f;

    public Transform LaneCenter => laneCenter;
    public Transform LeftLaneBound => leftLaneBound;
    public Transform RightLaneBound => rightLaneBound;
    public Transform CarSpawnPoint => carSpawnPoint;
    public float LaneWidth => laneWidth;
    public float RoadLength => roadLength;

    public void Configure(
        Transform laneCenterTransform,
        Transform leftLaneBoundTransform,
        Transform rightLaneBoundTransform,
        Transform carSpawnTransform,
        float configuredLaneWidth,
        float configuredRoadLength)
    {
        laneCenter = laneCenterTransform;
        leftLaneBound = leftLaneBoundTransform;
        rightLaneBound = rightLaneBoundTransform;
        carSpawnPoint = carSpawnTransform;
        laneWidth = configuredLaneWidth;
        roadLength = configuredRoadLength;
    }
}
