# # syntax=docker/dockerfile:1

FROM node:18.15-alpine as build-stage
WORKDIR /app

RUN rm -rf /usr/share/nginx/html/*
COPY package*.json /app/

RUN npm ci
COPY ./ /app/

RUN npm run build -- --output-path=./dist/ --configuration=production

FROM nginx:stable-alpine

COPY --from=build-stage /app/dist/ /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/nginx.conf

EXPOSE 80
EXPOSE 443

CMD ["nginx", "-g", "daemon off;"]
