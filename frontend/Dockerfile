FROM node:18-alpine3.15 AS builder
ARG BUILD_CONFIGURATION=development
WORKDIR /usr/src/frontend
RUN echo "Building a $BUILD_CONFIGURATION container"

COPY angular/src ./src
COPY angular/angular.json .
COPY angular/package.json .
COPY angular/package-lock.json .
COPY angular/tsconfig.app.json .
COPY angular/tsconfig.json .
COPY angular/tsconfig.spec.json .
RUN npm install -g @angular/cli && \
    npm install && \
    ng build --configuration $BUILD_CONFIGURATION

FROM openresty/openresty:1.21.4.1-0-alpine
COPY nginx/conf/nginx.conf /usr/local/openresty/nginx/conf/nginx.conf
COPY --from=builder /usr/src/frontend/dist/frontend /dist
