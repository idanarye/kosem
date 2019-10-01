* Server goes up.
* Procedure sends `LoginAsProcedure`.
    * Server returns the UUID.
* Procedure sends `RequestHuman`.
* Human sends `LoginAsHuman`.
    * Server replies with `LoginConfirmed`, that contains the role and a UUID.
* Server sends multiple `AvailableProcedure` requests - one for each procedure.
    * Should this be automatically, or should the human send something like `ListAvailableProcedures`?
* Human sends `JoinProcedure`.
* If the join is successful - both procedure and human receive `JoinConfirmation`.
* Once a procedure gets its human, all other humans will get `UnavailableProcedure`.
    * If a procedure disconnects all humans receive this.

Once enough humans have joined, the procedure can start the test. It is up to the test to wait for all procedures - it does not need to notify the server.

During a test, a procedure may:
    * Send `PushPhase` and receive a phase UID.
        * Other commands will use that phase UID.
    * Send `PopPhase` to remove a phase.
        * Need to decide - can we pop a phase from the middle of the stack?
        * Alternaitvely - pop **to** a phase, giving the UID of the deepest phase that will not be popped.
    * Send `AddComponent` to add a component to a phase:
        * `phase_uid` - the phase to add the component to.
        * `ordinal` field - optional, how to order the components.
        * `name` field - optional, sets a name for the component, so it can be modified later. Must be unique in the phase.
        * `type` field - `Caption`, `Text`, `Buttons` (may add more types in the future)
        * `params` field - the paramters of the phase, depending on the phase type.
    * Send `SetComponent` to modify an existing component:
        * Cannot modify the ordinal.
        * Can only work on named components.
        * Cannot change the `type` - only the `params`.
    * Send `RemoveComponent` to remove a named component.
    * Receive `ClickButton` when a human clicked a button.
        * The event will contain the name of the clicked button, as well as the
          values of all editable named fields in the phase.
