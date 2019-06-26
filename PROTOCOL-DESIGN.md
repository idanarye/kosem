* Server goes up.
* Testee sends `LoginAsTestee`.
    * Server replies with `LoginConfirmed`, that contains the role and a UUID.
* Testee sends `RequestTester`.
* Tester sends `LoginAsTester`.
    * Server replies with `LoginConfirmed`, that contains the role and a UUID.
* Server sends multiple `AvailableTestee` requests - one for each testee.
    * Should this be automatically, or should the tester send something like `ListAvailableTestees`?
* Tester sends `JoinTestee`.
* If the join is successful - both testee and tester receive `JoinConfirmation`.

Once enough testers have joined, the testee can start the test. It is up to the test to wait for all testees - it does not need to notify the server.

During a test, a testee may:
    * Send `PushPhase` and receive a phase ID.
        * All other commands must have a phase ID.
    * Send `PopPhase` to remove a phase.
        * Need to decide - can we pop a phase from the middle of the stack?
        * Alternaitvely - pop **to** a phase, giving the ID of the deepest phase that will not be popped.
    * Send `SetPhaseCaption` to add a caption to the phase.
    * Send `SetPhaseButtons` to add buttons to the phase.
        * Sets multiple buttons at once - so that the order is maintained.
    * Receive `ClickButton` when a tester clicked a button.

A tester may:
    * Receive the various phase commands that the testee sends:
        * `PushPhase` - add a phase component to the GUI.
        * `PopPhase` - remove a phase component from the GUI.
        * `SetPhaseCaption` - set the text on a phase GUI component.
        * `SetPhaseButtons` - set the buttons on a phase GUI component.
    * Send `ClickButton` when the user clicked a button.
        * The button will remained in a clicked state until the testee pops the phase.
