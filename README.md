# Rust Fantasy Football

## Overview
This is a sample project to explore building web APIs in Rust using a microservice
architecture.

## Services
API Gateway - the API gateway is the publicly accessible API that provides an interface to
the rest of the microservices

Players API - the Players API provides information about players and teams. This is a
read-heavy service. The idea is that the data in this service is fetched often and
does not change frequently. There is also not much data to store here so we can choose
a database instance that is faster but does not need to support having tons of data. We
may also decide to leverage a large amount of caching.
