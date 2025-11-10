# JavaScript Test Results
**Exit Code:** 0
**Total Files:** 2
**Passed:** 1
**Failed:** 1
**Success Rate:** 50.0%

**Breakdown:**
- Import failures: 1
- Runtime failures: 0

---

## Errors

1. `error: |-`
2. `Failed to import module: Cannot find module '/opt/todozi/tdz_js/todozi/todozi/agent.js' imported from /opt/todozi/tdz_js/todozi/server.js`

---

## Test Output

```
TAP version 13
# Subtest: file 1: /opt/todozi/server.js
not ok 1 - file 1: /opt/todozi/server.js
  ---
  duration_ms: 32.917243
  location: 'file:///tmp/js_test_harness_c61ar40q/harness.mjs:5:1'
  failureType: 'testCodeFailure'
  error: |-
    Failed to import module: Cannot find module '/opt/todozi/tdz_js/todozi/todozi/agent.js' imported from /opt/todozi/tdz_js/todozi/server.js
    Did you mean to import ../agent.js?
  code: 'ERR_ASSERTION'
  name: 'AssertionError'
  operator: 'fail'
  stack: |-
    TestContext.<anonymous> (file:///tmp/js_test_harness_c61ar40q/harness.mjs:10:12)
    async Test.run (node:internal/test_runner/test:632:9)
    async startSubtest (node:internal/test_runner/harness:214:3)
  ...
# Subtest: file 2: /opt/todozi/tdz.js
ok 2 - file 2: /opt/todozi/tdz.js
  ---
  duration_ms: 9.573848
  ...
Note: Using mock embedding service (full features may be limited)
                                                                                
                          _______        _                                      
                         |__   __|      | |        (✓)                          
                             | | ___   __| | ___ _____                          
                            | |/ _ \ / _` |/ _ \_  / |                          
                            | | (_) | (_| | (_) / /| |                          
                            |_|\___/ \__,_|\___/___|_|                          
                       ✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓                        
                                  Loading workspace...                          
                                      Please wait                               
                                                                                
[2J[HTodozi [✓] | 📁 Projects | 📋 Tasks | ✅ Done | 🔍 Find | 🔮 More | 🔑 API | 📰 Feed | 👋 Bye
────────────────────────────────────────────────────────────────────────────────
Projects-                                                                       
                                                                                
  Active (0)                                                                    
  + Add New Project                                                             
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
────────────────────────────────────────────────────────────────────────────────
Commands: Tab/←→ Switch tabs | q/ESC Exit                                       
[?25l[2J[H
👋 Goodbye!


```

---

## Files Tested

1. `tdz_js/server.js`
2. `tdz_js/tdz.js`
