# # syntax=docker/dockerfile:1

# # Comments are provided throughout this file to help you get started.
# # If you need more help, visit the Dockerfile reference guide at
# # https://docs.docker.com/go/dockerfile-reference/

# # Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

# ARG NODE_VERSION=21.0.0
# ARG PNPM_VERSION=9.9.0

# ################################################################################
# # Use node image for base image for all stages.
# FROM node:${NODE_VERSION}-alpine as base

# # Set working directory for all build stages.
# WORKDIR /usr/src/app

# # Install pnpm.
# RUN --mount=type=cache,target=/root/.npm \
#     npm install -g pnpm@${PNPM_VERSION}

# ################################################################################
# # Create a stage for installing production dependecies.
# FROM base as deps

# # Download dependencies as a separate step to take advantage of Docker's caching.
# # Leverage a cache mount to /root/.local/share/pnpm/store to speed up subsequent builds.
# # Leverage bind mounts to package.json and pnpm-lock.yaml to avoid having to copy them
# # into this layer.
# RUN --mount=type=bind,source=package.json,target=package.json \
#     --mount=type=bind,source=pnpm-lock.yaml,target=pnpm-lock.yaml \
#     --mount=type=cache,target=/root/.local/share/pnpm/store \
#     pnpm install --prod --frozen-lockfile

# ################################################################################
# # Create a stage for building the application.
# FROM deps as build

# # Download additional development dependencies before building, as some projects require
# # "devDependencies" to be installed to build. If you don't need this, remove this step.
# RUN --mount=type=bind,source=package.json,target=package.json \
#     --mount=type=bind,source=pnpm-lock.yaml,target=pnpm-lock.yaml \
#     --mount=type=cache,target=/root/.local/share/pnpm/store \
#     pnpm install --frozen-lockfile

# # Copy the rest of the source files into the image.
# COPY . .
# # Run the build script.
# RUN pnpm run build --configuration=production

# ################################################################################
# # Create a new stage to run the application with minimal runtime dependencies
# # where the necessary files are copied from the build stage.
# FROM nginx:stable-alpine as final

# # Use production node environment by default.
# ENV NODE_ENV production

# # Copy package.json so that package manager commands can be used.
# COPY package.json .

# # Copy the production dependencies from the deps stage and also
# # the built application from the build stage into the image.
# COPY --from=deps /usr/src/app/node_modules ./node_modules
# COPY --from=build /usr/src/app/dist /usr/share/nginx/html

# COPY ./nginx.conf /etc/nginx/nginx.conf

# EXPOSE 80
# EXPOSE 443

# CMD ["nginx", "-g", "daemon off;"]

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


