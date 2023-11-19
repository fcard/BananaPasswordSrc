def Settings(**kwargs):
  if kwargs['language'] == 'rust':
    return {
      'ls': {
        'cargo': {
          'buildScripts': True,
        },
        'procMacro': {
          'enable': True,
          'attributes': {
            'enable': True,
          },
        },
        'diagnostics': {
          'disabled': ['unresolved-proc-macro', 'inactive-code'],
        },
      }
    }

