### Kalman Filter

The Kalman filter is a state estimator that makes an estimate of some unobserved variable based on noisy measurements.
It is a recursive filter that is based on the assumption that the state of the system can be described by a linear 
stochastic difference equation. The Kalman filter is a powerful tool for combining information in the presence of uncertainty.
It is used in a wide range of applications including target tracking, guidance and navigation systems, radar data processing, and satellite orbit determination.

For our case we need to estimate the position of the rocket, based on the measurements from the IMU, pressure sensor and magnetometer.
In the case of system shutdown, we need to be able to estimate the position of the rocket, based on the last known position and the
last known velocity. This is where the Kalman filter comes in. It is a recursive filter, meaning that it can estimate the state of the system
based on the previous state and the current measurement.
The previous state is stored in an SD-card

