* Server goes up.
* Procedure sends `LoginAsProcedure`.
    * Server replies with `LoginConfirmed`, that contains the role and a UUID.
* Procedure sends `RequestHuman`.
* Human sends `LoginAsHuman`.
    * Server replies with `LoginConfirmed`, that contains the role and a UUID.
* Server sends multiple `AvailableProcedure` requests - one for each procedure.
    * Should this be automatically, or should the human send something like `ListAvailableProcedures`?
* Human sends `JoinProcedure`.
* If the join is successful - both procedure and human receive `JoinConfirmation`.

Once enough humans have joined, the procedure can start the test. It is up to the test to wait for all procedures - it does not need to notify the server.

During a test, a procedure may:
    * Send `PushPhase` and receive a phase ID.
        * All other commands must have a phase ID.
    * Send `PopPhase` to remove a phase.
        * Need to decide - can we pop a phase from the middle of the stack?
        * Alternaitvely - pop **to** a phase, giving the ID of the deepest phase that will not be popped.
    * Send `SetPhaseCaption` to add a caption to the phase.
    * Send `SetPhaseButtons` to add buttons to the phase.
        * Sets multiple buttons at once - so that the order is maintained.
    * Receive `ClickButton` when a human clicked a button.

A human may:
    * Receive the various phase commands that the procedure sends:
        * `PushPhase` - add a phase component to the GUI.
        * `PopPhase` - remove a phase component from the GUI.
        * `SetPhaseCaption` - set the text on a phase GUI component.
        * `SetPhaseButtons` - set the buttons on a phase GUI component.
    * Send `ClickButton` when the user clicked a button.
        * The button will remained in a clicked state until the procedure pops the phase.
