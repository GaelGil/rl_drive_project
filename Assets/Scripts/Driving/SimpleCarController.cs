using UnityEngine;

[RequireComponent(typeof(Rigidbody))]
public class SimpleCarController : MonoBehaviour
{
    [SerializeField] private float acceleration = 12f;
    [SerializeField] private float maxSpeed = 18f;
    [SerializeField] private float steeringDegrees = 90f;

    private Rigidbody body;

    private void Awake()
    {
        body = GetComponent<Rigidbody>();
    }

    private void Reset()
    {
        ConfigureBody();
    }

    private void OnValidate()
    {
        ConfigureBody();
    }

    private void FixedUpdate()
    {
        if (body == null)
        {
            body = GetComponent<Rigidbody>();
        }

        float throttle = Mathf.Clamp01(Input.GetAxis("Vertical"));
        float steering = Input.GetAxis("Horizontal");

        float forwardSpeed = Vector3.Dot(body.linearVelocity, transform.forward);
        float targetSpeed = Mathf.Clamp(forwardSpeed + throttle * acceleration * Time.fixedDeltaTime, 0f, maxSpeed);
        Quaternion steeringStep = Quaternion.Euler(0f, steering * steeringDegrees * Time.fixedDeltaTime, 0f);

        body.MoveRotation(body.rotation * steeringStep);

        Vector3 nextVelocity = transform.forward * targetSpeed;
        nextVelocity.y = body.linearVelocity.y;
        body.linearVelocity = nextVelocity;
    }

    private void ConfigureBody()
    {
        if (!TryGetComponent(out Rigidbody attachedBody))
        {
            return;
        }

        body = attachedBody;
        body.useGravity = true;
        body.linearDamping = 0.75f;
        body.angularDamping = 3f;
        body.constraints = RigidbodyConstraints.FreezeRotationX | RigidbodyConstraints.FreezeRotationZ;
    }
}
