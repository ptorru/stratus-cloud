# This declares a job named "docs". There can be exactly one
# job declaration per job file.
job "ambient-prom" {
  affinity {    
    attribute = "${node.id}"    
    value     = "17630daa-2df1-6720-12cc-7924d50caae6"    
    weight    = 100  
  }

  name = "ambient-prom"

  # Spread the tasks in this job between us-west-1 and us-east-1.
  datacenters = ["dc1"]

  # Run this job as a "service" type. Each job type has different
  # properties. See the documentation below for more examples.
  type = "service"


  # A group defines a series of tasks that should be co-located
  # on the same client (host). All tasks within a group will be
  # placed on the same host.
  group "exporters" {
    # Specify the number of these tasks we want.
    count = 1



    # Create an individual task (unit of work). This particular
    # task utilizes a Docker container to front a web application.
    task "ambient-exporter" {

      driver = "exec"

      config {
        command = "/usr/bin/ambient_prom"
        pid_mode = "host"
        ipc_mode = "host"

      }

      # Specify the maximum resources required to run the task,
      # include CPU and memory.
      resources {
        cpu    = 1000 # MHz
        memory = 256 # MB
        network {
          mbits = 100
          #port "http" {}
          port "prom" {
            static = 9186
          }
        }
      }
    }
  }
}
