using UnityEngine;

[ExecuteAlways]
public class HighwayBootstrapper : MonoBehaviour
{
    [SerializeField] private float laneWidth = 4f;
    [SerializeField] private float roadLength = 120f;
    [SerializeField] private float roadWidth = 12f;
    [SerializeField] private float shoulderWidth = 2f;
    [SerializeField] private int obstacleCount = 4;
    [SerializeField] private HighwayRoadReferences roadReferences;

    private void OnEnable()
    {
        EnsureEnvironment();
    }

    private void OnValidate()
    {
        EnsureEnvironment();
    }

    private void EnsureEnvironment()
    {
        if (!isActiveAndEnabled)
        {
            return;
        }

        Transform road = EnsurePrimitive(
            "Road",
            transform,
            new Vector3(0f, -0.1f, 0f),
            new Vector3(roadWidth, 0.2f, roadLength));

        EnsurePrimitive(
            "LeftBoundary",
            transform,
            new Vector3(-(roadWidth * 0.5f + shoulderWidth * 0.5f), 0.5f, 0f),
            new Vector3(shoulderWidth, 1f, roadLength));

        EnsurePrimitive(
            "RightBoundary",
            transform,
            new Vector3(roadWidth * 0.5f + shoulderWidth * 0.5f, 0.5f, 0f),
            new Vector3(shoulderWidth, 1f, roadLength));

        Transform laneCenter = EnsureEmpty("LaneCenter", transform, Vector3.zero);
        Transform leftLaneBound = EnsureEmpty("LeftLaneBound", transform, new Vector3(-laneWidth * 0.5f, 0f, 0f));
        Transform rightLaneBound = EnsureEmpty("RightLaneBound", transform, new Vector3(laneWidth * 0.5f, 0f, 0f));
        Transform carSpawn = EnsureEmpty(
            "CarSpawnPoint",
            transform,
            new Vector3(0f, 0.6f, -roadLength * 0.5f + 12f));

        EnsurePrimitive(
            "LaneCenterVisual",
            road,
            new Vector3(0f, 0.61f, 0f),
            new Vector3(0.12f, 0.02f, roadLength));

        EnsurePrimitive(
            "LeftLaneVisual",
            road,
            new Vector3(-laneWidth * 0.5f, 0.61f, 0f),
            new Vector3(0.08f, 0.02f, roadLength));

        EnsurePrimitive(
            "RightLaneVisual",
            road,
            new Vector3(laneWidth * 0.5f, 0.61f, 0f),
            new Vector3(0.08f, 0.02f, roadLength));

        Transform car = EnsurePrimitive(
            "PlayerCar",
            transform,
            carSpawn.localPosition,
            new Vector3(1.8f, 1f, 4f));

        Rigidbody carBody = EnsureComponent<Rigidbody>(car.gameObject);
        carBody.mass = 1200f;

        EnsureComponent<SimpleCarController>(car.gameObject);

        Transform obstacleRoot = EnsureEmpty("Obstacles", transform, Vector3.zero);
        float[] laneOffsets = { 0f, -laneWidth * 0.3f, laneWidth * 0.3f, 0f, -laneWidth * 0.2f, laneWidth * 0.2f };
        float startZ = -roadLength * 0.2f;
        float spacing = 20f;

        for (int index = 0; index < obstacleCount; index++)
        {
            Transform obstacle = EnsurePrimitive(
                $"Obstacle_{index + 1:00}",
                obstacleRoot,
                new Vector3(laneOffsets[index % laneOffsets.Length], 0.75f, startZ + spacing * index),
                new Vector3(1.8f, 1.5f, 2.5f));

            obstacle.localRotation = Quaternion.identity;
        }

        roadReferences = EnsureComponent<HighwayRoadReferences>(gameObject);
        roadReferences.Configure(laneCenter, leftLaneBound, rightLaneBound, carSpawn, laneWidth, roadLength);
    }

    private static Transform EnsureEmpty(string objectName, Transform parent, Vector3 localPosition)
    {
        Transform child = parent.Find(objectName);
        if (child == null)
        {
            child = new GameObject(objectName).transform;
            child.SetParent(parent, false);
        }

        child.localPosition = localPosition;
        child.localRotation = Quaternion.identity;
        child.localScale = Vector3.one;
        return child;
    }

    private static Transform EnsurePrimitive(string objectName, Transform parent, Vector3 localPosition, Vector3 localScale)
    {
        Transform child = parent.Find(objectName);
        if (child == null)
        {
            GameObject primitive = GameObject.CreatePrimitive(PrimitiveType.Cube);
            primitive.name = objectName;
            child = primitive.transform;
            child.SetParent(parent, false);
        }

        child.localPosition = localPosition;
        child.localRotation = Quaternion.identity;
        child.localScale = localScale;
        return child;
    }

    private static T EnsureComponent<T>(GameObject target) where T : Component
    {
        if (!target.TryGetComponent(out T component))
        {
            component = target.AddComponent<T>();
        }

        return component;
    }
}
