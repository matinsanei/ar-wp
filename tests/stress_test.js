import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '30s', target: 1000 }, // Ramp-up to 1000 concurrent users
    { duration: '3m',  target: 1000 }, // Sustain peak load of 1000 concurrent users
    { duration: '30s', target: 0 },    // Ramp-down to 0
  ],
  thresholds: {
    http_req_failed: ['rate<0.05'], // Less than 5% failure under extreme stress
    http_req_duration: ['p(95)<15000'], // 95% of requests under 15s under 1000 concurrent users
  },
};

export default function () {
  // Test both dynamic full WordPress page and high-speed assets
  const endpoints = [
    'http://127.0.0.1:8080/',
    'http://127.0.0.1:8080/wp-admin/css/common.min.css',
  ];

  const target = endpoints[Math.floor(Math.random() * endpoints.length)];
  const res = http.get(target);

  check(res, {
    'status is 200': (r) => r.status === 200,
  });

  sleep(0.2);
}
