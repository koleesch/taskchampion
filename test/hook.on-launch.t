#! /usr/bin/perl
################################################################################
## taskwarrior - a command line task list manager.
##
## Copyright 2006-2011, Paul Beckingham, Federico Hernandez.
##
## Permission is hereby granted, free of charge, to any person obtaining a copy
## of this software and associated documentation files (the "Software"), to deal
## in the Software without restriction, including without limitation the rights
## to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
## copies of the Software, and to permit persons to whom the Software is
## furnished to do so, subject to the following conditions:
##
## The above copyright notice and this permission notice shall be included
## in all copies or substantial portions of the Software.
##
## THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
## OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
## FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
## THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
## LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
## OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
## SOFTWARE.
##
## http://www.opensource.org/licenses/mit-license.php
##
################################################################################

use strict;
use warnings;
use Test::More tests => 10;

# Create the rc file.
if (open my $fh, '>', 'hook.rc')
{
  print $fh "data.location=.\n",
            "extensions=on\n",
            "hook.on-launch=" . $ENV{'PWD'} . "/hook:test\n";
  close $fh;
  ok (-r 'hook.rc', 'Created hook.rc');
}

if (open my $fh, '>', 'hook')
{
  print $fh "function test () print ('marker') return 0, nil end\n";
  close $fh;
  ok (-r 'hook', 'Created hook');
}

my $output = qx{../src/task rc:hook.rc version};
if ($output =~ /PUC-Rio/)
{
  # Test the hook.
  $output = qx{../src/task rc:hook.rc _version};
  like ($output, qr/^marker.+\n\w{7}/ms, 'Found marker before output');
}
else
{
  pass ('Found marker before output - skip: no Lua support');
}

# Cleanup.
unlink 'pending.data';
ok (!-r 'pending.data', 'Removed pending.data');

unlink 'completed.data';
ok (!-r 'completed.data', 'Removed completed.data');

unlink 'undo.data';
ok (!-r 'undo.data', 'Removed undo.data');

unlink 'backlog.data';
ok (!-r 'backlog.data', 'Removed backlog.data');

unlink 'synch.key';
ok (!-r 'synch.key', 'Removed synch.key');

unlink 'hook';
ok (!-r 'hook', 'Removed hook');

unlink 'hook.rc';
ok (!-r 'hook.rc', 'Removed hook.rc');

exit 0;

