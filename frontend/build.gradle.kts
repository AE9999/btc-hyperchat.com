plugins {
  id("com.bmuschko.docker-remote-api") version "6.7.0"
}

// Import task types
import com.bmuschko.gradle.docker.tasks.image.*

val environment: String = if (System.getenv("FRONTEND_BUILD_CONFIGURATION").isNullOrBlank()) {
                      "development"
                  } else {
                    System.getenv("FRONTEND_BUILD_CONFIGURATION")
                  };

tasks.create("buildDocker", DockerBuildImage::class) {
    inputDir.set(file("${projectDir}"))
    buildArgs.set(mapOf("BUILD_CONFIGURATION" to environment))
    images.add("btc-chat-frontend")
}

