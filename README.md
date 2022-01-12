rwatch [regex pattern to watch] [command to run]
i.e. rwatch '.*\.rs' 'echo {} && echo {}'

Note: regex needs escaping or unpredictable results may happen.

rwatch will match on **all** modification events. This includes updating timestamps.
